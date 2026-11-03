mod word;
use resume_domain::{FontFamily, PageSize, VisualBlock, VisualDocument};
use std::{fs::File, io::Write, path::Path};
use tempfile::NamedTempFile;
use zip::{write::SimpleFileOptions, ZipWriter};

const WORD_NS: &str = "http://schemas.openxmlformats.org/wordprocessingml/2006/main";
const REL_NS: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";

pub fn export_html_bundle(
    path: &Path,
    title: &str,
    document: &VisualDocument,
    css: &str,
) -> Result<(), String> {
    validate_document(document)?;
    let article = render_html_article(document);
    let page_title = escape_html(title);
    let html = format!(
        "<!doctype html>\n<html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><title>{page_title}</title><link rel=\"stylesheet\" href=\"resume.css\"></head><body>{article}</body></html>\n"
    );
    let file = atomic_zip(path, |zip| {
        add_zip_entry(zip, "index.html", html.as_bytes())?;
        add_zip_entry(zip, "resume.css", css.as_bytes())
    })?;
    file.sync_all()
        .map_err(|error| format!("sync HTML bundle: {error}"))
}

pub fn export_docx(path: &Path, document: &VisualDocument) -> Result<(), String> {
    export_docx_with_mode(path, document, DocxMode::Standard)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DocxMode {
    Standard,
    Design,
}

pub fn export_docx_with_mode(
    path: &Path,
    document: &VisualDocument,
    mode: DocxMode,
) -> Result<(), String> {
    validate_document(document)?;
    let mut relationships = Vec::new();
    let document_xml = match mode {
        DocxMode::Standard => render_standard_document_xml(document, &mut relationships),
        DocxMode::Design => render_document_xml(document, &mut relationships),
    };
    let tokens: serde_json::Value =
        serde_json::from_str(include_str!("../../../design/resume-layout.json"))
            .map_err(|error| format!("read bundled layout tokens: {error}"))?;
    let styles_xml = render_styles_xml(document, &tokens);
    let relationships_xml = render_relationships_xml(&relationships);
    let content_types = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Default Extension="xml" ContentType="application/xml"/><Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/><Override PartName="/word/styles.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml"/></Types>"#;
    let root_relationships = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml"/></Relationships>"#;
    let file = atomic_zip(path, |zip| {
        add_zip_entry(zip, "[Content_Types].xml", content_types.as_bytes())?;
        add_zip_entry(zip, "_rels/.rels", root_relationships.as_bytes())?;
        add_zip_entry(zip, "word/document.xml", document_xml.as_bytes())?;
        add_zip_entry(zip, "word/styles.xml", styles_xml.as_bytes())?;
        add_zip_entry(
            zip,
            "word/_rels/document.xml.rels",
            relationships_xml.as_bytes(),
        )
    })?;
    file.sync_all()
        .map_err(|error| format!("sync DOCX file: {error}"))
}

fn validate_document(document: &VisualDocument) -> Result<(), String> {
    if document.schema_version != resume_domain::VISUAL_DOCUMENT_SCHEMA_VERSION {
        return Err(format!(
            "unsupported visual document version {}",
            document.schema_version
        ));
    }
    if document.blocks.is_empty() {
        return Err("cannot export an empty resume".into());
    }
    if !valid_color(&document.style.accent_color) {
        return Err("resume accent color is invalid".into());
    }
    if let Some(presentation) = &document.style.presentation {
        if !valid_color(&presentation.surface)
            || !valid_color(&presentation.rule)
            || !valid_color(&presentation.dark)
        {
            return Err("resume palette contains an invalid color".into());
        }
    }
    let text_size: usize = document
        .blocks
        .iter()
        .map(|block| match block {
            VisualBlock::Identity {
                title,
                subtitle,
                eyebrow,
                contact,
            } => {
                title.len()
                    + subtitle.as_deref().unwrap_or("").len()
                    + eyebrow.as_deref().unwrap_or("").len()
                    + contact.as_deref().unwrap_or("").len()
            }
            VisualBlock::Section { title, entries, .. } => {
                title.len()
                    + entries
                        .iter()
                        .map(|entry| {
                            entry.heading.len()
                                + entry.metadata.iter().map(String::len).sum::<usize>()
                                + entry.body.len()
                        })
                        .sum::<usize>()
            }
        })
        .sum();
    if text_size > 5 * 1024 * 1024 {
        return Err("resume content exceeds the 5 MiB export limit".into());
    }
    Ok(())
}

fn valid_color(value: &str) -> bool {
    value.len() == 7
        && value.starts_with('#')
        && value[1..].bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn render_html_article(document: &VisualDocument) -> String {
    let style = &document.style;
    let font = match style.font_family {
        FontFamily::Serif => "Georgia, serif",
        FontFamily::SansSerif => "Liberation Sans, Arial, sans-serif",
    };
    let presentation = style.presentation.as_ref();
    let palette = if presentation.is_some_and(|value| !value.surface.is_empty()) {
        "selected"
    } else {
        "default"
    };
    let decoration = presentation
        .and_then(|value| value.heading_decoration)
        .map(|value| match value {
            resume_domain::HeadingDecoration::Rule => "rule",
            resume_domain::HeadingDecoration::Minimal => "minimal",
        })
        .unwrap_or("default");
    let heading_size = style.body_size_pt as u16 * style.heading_scale_percent as u16 / 100;
    let mut inline_style = format!("--resume-accent:{};--resume-body-size:{}pt;--resume-heading-scale:{}%;--resume-heading-size:{}pt;--resume-font:{}", style.accent_color, style.body_size_pt, style.heading_scale_percent, heading_size, font);
    if let Ok(tokens) = serde_json::from_str::<serde_json::Value>(include_str!(
        "../../../design/resume-layout.json"
    )) {
        let line_key = if style.density == resume_domain::Density::Compact {
            "compactLineHeight"
        } else {
            "comfortableLineHeight"
        };
        inline_style.push_str(&format!(";--resume-section-size:{}pt;--resume-line-height:{};--resume-entry-gap:{}mm;--resume-paragraph-gap:{}mm", tokens["sectionSizePt"], tokens[line_key], tokens["entryGapMm"], tokens["paragraphGapMm"]));
    }
    if let Some(value) = presentation {
        if !value.surface.is_empty() {
            inline_style.push_str(&format!(";--resume-surface:{}", value.surface));
        }
        if !value.rule.is_empty() {
            inline_style.push_str(&format!(";--resume-rule:{}", value.rule));
        }
        if !value.dark.is_empty() {
            inline_style.push_str(&format!(";--resume-dark:{}", value.dark));
        }
    }
    let mut html = format!("<article id=\"resume-preview\" class=\"resume template-{} density-{}\" data-layout=\"{}\" data-palette=\"{palette}\" data-decoration=\"{decoration}\" style=\"{}\" aria-label=\"Resume\">", document.template_id, density_name(style.density), layout_name(document.layout), escape_html(&inline_style));
    let mut main = String::new();
    let mut side = String::new();
    let mut linear = String::new();
    for block in &document.blocks {
        match block {
            VisualBlock::Identity {
                title,
                subtitle,
                eyebrow,
                contact,
            } => {
                html.push_str("<header class=\"resume-identity\"><h2>");
                html.push_str(&escape_html(title));
                html.push_str("</h2>");
                append_html_paragraph(&mut html, "resume-subtitle", subtitle.as_deref());
                append_html_paragraph(&mut html, "resume-eyebrow", eyebrow.as_deref());
                append_html_paragraph(&mut html, "resume-contact", contact.as_deref());
                html.push_str("</header>");
            }
            VisualBlock::Section {
                kind,
                title,
                entries,
            } => {
                if entries.is_empty() {
                    continue;
                }
                let kind_attr = kind
                    .map(|value| format!(" data-kind=\"{}\"", section_kind_name(value)))
                    .unwrap_or_default();
                let mut section = format!(
                    "<section class=\"resume-section\"{kind_attr}><h3>{}</h3>",
                    escape_html(title)
                );
                for entry in entries {
                    section.push_str("<article class=\"resume-entry\"><h4>");
                    section.push_str(&escape_html(&entry.heading));
                    section.push_str("</h4>");
                    if !entry.metadata.is_empty() {
                        section.push_str("<p class=\"resume-metadata\">");
                        section.push_str(&html_text_with_links(&entry.metadata.join(" · ")));
                        section.push_str("</p>");
                    }
                    if !entry.body.trim().is_empty() {
                        section.push_str("<p class=\"resume-body\">");
                        section.push_str(&html_text_with_links(&entry.body));
                        section.push_str("</p>");
                    }
                    section.push_str("</article>");
                }
                section.push_str("</section>");
                linear.push_str(&section);
                if kind.is_some_and(|value| is_side_section(&value)) {
                    side.push_str(&section);
                } else {
                    main.push_str(&section);
                }
            }
        }
    }
    if document.layout == resume_domain::Layout::TwoColumn && !main.is_empty() && !side.is_empty() {
        html.push_str("<div class=\"resume-columns\"><div class=\"resume-column-main\">");
        html.push_str(&main);
        html.push_str("</div><aside class=\"resume-column-side\">");
        html.push_str(&side);
        html.push_str("</aside></div>");
    } else if document.layout == resume_domain::Layout::TwoColumn && main.is_empty() {
        html.push_str("<div class=\"resume-columns is-single-column columns-only-side\"><aside class=\"resume-column-side\">");
        html.push_str(&side);
        html.push_str("</aside></div>");
    } else if document.layout == resume_domain::Layout::TwoColumn && side.is_empty() {
        html.push_str("<div class=\"resume-columns is-single-column columns-only-main\"><div class=\"resume-column-main\">");
        html.push_str(&main);
        html.push_str("</div></div>");
    } else {
        html.push_str(&linear);
    }
    html.push_str("</article>");
    html
}

fn density_name(value: resume_domain::Density) -> &'static str {
    match value {
        resume_domain::Density::Compact => "compact",
        resume_domain::Density::Comfortable => "comfortable",
    }
}

fn layout_name(value: resume_domain::Layout) -> &'static str {
    match value {
        resume_domain::Layout::SingleColumn => "single_column",
        resume_domain::Layout::AccentHeader => "accent_header",
        resume_domain::Layout::CompactStack => "compact_stack",
        resume_domain::Layout::TwoColumn => "two_column",
    }
}

fn section_kind_name(value: resume_domain::SectionKind) -> &'static str {
    use resume_domain::SectionKind::*;
    match value {
        Summary => "summary",
        Experience => "experience",
        Education => "education",
        Project => "project",
        Skill => "skill",
        Certification => "certification",
        Language => "language",
        Award => "award",
        Volunteering => "volunteering",
        Publication => "publication",
        Membership => "membership",
        License => "license",
        Reference => "reference",
        Interest => "interest",
    }
}

fn is_side_section(value: &resume_domain::SectionKind) -> bool {
    matches!(
        value,
        resume_domain::SectionKind::Education
            | resume_domain::SectionKind::Skill
            | resume_domain::SectionKind::Certification
            | resume_domain::SectionKind::Language
            | resume_domain::SectionKind::Membership
            | resume_domain::SectionKind::License
            | resume_domain::SectionKind::Reference
            | resume_domain::SectionKind::Interest
    )
}

fn append_html_paragraph(html: &mut String, class: &str, text: Option<&str>) {
    if let Some(text) = text.filter(|text| !text.is_empty()) {
        html.push_str(&format!(
            "<p class=\"{class}\">{}</p>",
            html_text_with_links(text)
        ));
    }
}

fn html_text_with_links(text: &str) -> String {
    let mut output = String::new();
    let mut cursor = 0;
    while let Some(relative) = text[cursor..]
        .find("http://")
        .or_else(|| text[cursor..].find("https://"))
    {
        let start = cursor + relative;
        output.push_str(&escape_html(&text[cursor..start]));
        let end = text[start..]
            .find(char::is_whitespace)
            .map(|offset| start + offset)
            .unwrap_or(text.len());
        let raw = &text[start..end];
        let link_end = raw.trim_end_matches(['.', ',', ';', ')', ']', '}']).len();
        if link_end == 0 {
            output.push_str(&escape_html(&text[start..end]));
        } else {
            let url = escape_html(&raw[..link_end]);
            output.push_str(&format!(
                "<a href=\"{url}\" rel=\"noreferrer noopener\">{url}</a>"
            ));
            output.push_str(&escape_html(&raw[link_end..]));
        }
        cursor = end;
    }
    output.push_str(&escape_html(&text[cursor..]));
    output
}

fn atomic_zip<F>(path: &Path, write_entries: F) -> Result<File, String>
where
    F: FnOnce(&mut ZipWriter<NamedTempFile>) -> Result<(), String>,
{
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let temporary =
        NamedTempFile::new_in(parent).map_err(|error| format!("create export file: {error}"))?;
    let mut archive = ZipWriter::new(temporary);
    write_entries(&mut archive)?;
    let temporary = archive
        .finish()
        .map_err(|error| format!("finish export archive: {error}"))?;
    temporary
        .as_file()
        .sync_all()
        .map_err(|error| format!("sync export archive: {error}"))?;
    let file = temporary
        .persist(path)
        .map_err(|error| format!("replace export file: {}", error.error))?;
    Ok(file)
}

fn add_zip_entry(
    zip: &mut ZipWriter<NamedTempFile>,
    name: &str,
    bytes: &[u8],
) -> Result<(), String> {
    zip.start_file(name, SimpleFileOptions::default())
        .map_err(|error| format!("create {name} in export archive: {error}"))?;
    zip.write_all(bytes)
        .map_err(|error| format!("write {name}: {error}"))
}

fn render_standard_document_xml(
    document: &VisualDocument,
    links: &mut Vec<(String, String)>,
) -> String {
    let mut body = String::new();
    for block in &document.blocks {
        match block {
            VisualBlock::Identity {
                title,
                subtitle,
                eyebrow,
                contact,
            } => {
                body.push_str(&paragraph(title, "Title", ""));
                for (value, style) in [
                    (subtitle, "Subtitle"),
                    (eyebrow, "Metadata"),
                    (contact, "Metadata"),
                ] {
                    if let Some(value) = value.as_deref().filter(|value| !value.is_empty()) {
                        body.push_str(&paragraph_with_links(value, style, links));
                    }
                }
            }
            VisualBlock::Section {
                kind,
                title,
                entries,
            } => {
                if entries.is_empty() {
                    continue;
                }
                body.push_str(&section_heading(
                    title,
                    document.style.accent_color.trim_start_matches('#'),
                    "D8E0E8",
                ));
                for entry in entries {
                    if *kind != Some(resume_domain::SectionKind::Summary) {
                        body.push_str(&paragraph(&entry.heading, "Heading2", ""));
                    }
                    if !entry.metadata.is_empty() {
                        body.push_str(&paragraph_with_links(
                            &entry.metadata.join(" · "),
                            "Metadata",
                            links,
                        ));
                    }
                    if !entry.body.trim().is_empty() {
                        body.push_str(&paragraph_with_links(&entry.body, "Normal", links));
                    }
                }
            }
        }
    }
    let (width, height) = page_dimensions(document.page_size);
    format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><w:document xmlns:w="{WORD_NS}" xmlns:r="{REL_NS}"><w:body>{body}<w:sectPr><w:pgSz w:w="{width}" w:h="{height}"/><w:pgMar w:top="850" w:right="850" w:bottom="850" w:left="850" w:header="360" w:footer="360" w:gutter="0"/></w:sectPr></w:body></w:document>"#
    )
}

fn render_document_xml(document: &VisualDocument, links: &mut Vec<(String, String)>) -> String {
    use resume_domain::Layout;

    let accent = document.style.accent_color.trim_start_matches('#');
    let surface = document
        .style
        .presentation
        .as_ref()
        .map(|style| style.surface.trim_start_matches('#'))
        .unwrap_or("EDF2F6");
    let dark = document
        .style
        .presentation
        .as_ref()
        .map(|style| style.dark.trim_start_matches('#'))
        .unwrap_or("18364A");
    let has_main_sections = document.blocks.iter().any(|block| match block {
        VisualBlock::Section { kind, entries, .. } => {
            !entries.is_empty() && !kind.is_some_and(|value| is_side_section(&value))
        }
        VisualBlock::Identity { .. } => false,
    });
    let has_side_sections = document.blocks.iter().any(|block| match block {
        VisualBlock::Section { kind, entries, .. } => {
            !entries.is_empty() && kind.is_some_and(|value| is_side_section(&value))
        }
        VisualBlock::Identity { .. } => false,
    });
    let render_as_columns =
        document.layout == Layout::TwoColumn && has_main_sections && has_side_sections;
    let mut identity = String::new();
    let mut main_sections = Vec::new();
    let mut side_sections = Vec::new();
    let mut first_section = true;
    for block in &document.blocks {
        match block {
            VisualBlock::Identity {
                title,
                subtitle,
                eyebrow,
                contact,
            } => {
                identity = render_docx_identity(
                    document,
                    title,
                    subtitle.as_deref(),
                    eyebrow.as_deref(),
                    contact.as_deref(),
                    surface,
                    dark,
                    accent,
                    page_dimensions(document.page_size).0 - 1440,
                    links,
                );
            }
            VisualBlock::Section {
                kind,
                title,
                entries,
            } => {
                if entries.is_empty() {
                    continue;
                }
                let panel = (first_section
                    && matches!(
                        document.template_id.as_str(),
                        "modern_business" | "creative_minimal"
                    ))
                    || (*kind == Some(resume_domain::SectionKind::Summary)
                        && matches!(
                            document.template_id.as_str(),
                            "modern" | "creative_design" | "executive_leadership"
                        ));
                first_section = false;
                let section = render_docx_section(
                    document,
                    *kind,
                    title,
                    entries,
                    render_as_columns
                        && document.template_id == "two_column_sidebar"
                        && kind.is_some_and(|value| is_side_section(&value)),
                    panel,
                    surface,
                    accent,
                    page_dimensions(document.page_size).0 - 1440,
                    links,
                );
                if document.layout == Layout::TwoColumn
                    && kind.is_some_and(|value| is_side_section(&value))
                {
                    side_sections.push(section);
                } else {
                    main_sections.push(section);
                }
            }
        }
    }
    let mut body = identity;
    if render_as_columns {
        let (width, _) = page_dimensions(document.page_size);
        let table_width = width - 2 * 720;
        let side_width = if document.template_id == "two_column_sidebar" {
            table_width * 32 / 100
        } else {
            table_width * 31 / 100
        };
        let main_width = table_width - side_width;
        let (left, right) = if document.template_id == "two_column_sidebar" {
            (side_sections.join(""), main_sections.join(""))
        } else {
            (main_sections.join(""), side_sections.join(""))
        };
        let left_width = if document.template_id == "two_column_sidebar" {
            side_width
        } else {
            main_width
        };
        let right_width = table_width - left_width;
        let sidebar_cell = |content: String, width: u32, background: Option<&str>| {
            let cell_background = background
                .map(|color| format!("<w:shd w:val=\"clear\" w:fill=\"{color}\"/>"))
                .unwrap_or_default();
            format!("<w:tc><w:tcPr><w:tcW w:w=\"{width}\" w:type=\"dxa\"/><w:tcMar><w:top w:w=\"0\" w:type=\"dxa\"/><w:bottom w:w=\"0\" w:type=\"dxa\"/><w:start w:w=\"180\" w:type=\"dxa\"/><w:end w:w=\"180\" w:type=\"dxa\"/></w:tcMar>{cell_background}</w:tcPr>{content}</w:tc>")
        };
        let left_background = if document.template_id == "two_column_sidebar" {
            Some(dark)
        } else {
            None
        };
        let right_background = if document.template_id == "two_column_modern" {
            Some(surface)
        } else {
            None
        };
        let paragraphs = |content: &str| -> Vec<String> {
            content
                .split_inclusive("</w:p>")
                .filter(|block| !block.is_empty())
                .map(str::to_owned)
                .collect()
        };
        let left = paragraphs(&left);
        let right = paragraphs(&right);
        let mut rows = String::new();
        for index in 0..left.len().max(right.len()) {
            let left_content = left.get(index).cloned().unwrap_or_else(|| "<w:p/>".into());
            let right_content = right.get(index).cloned().unwrap_or_else(|| "<w:p/>".into());
            rows.push_str(&format!(
                "<w:tr><w:trPr><w:cantSplit w:val=\"0\"/></w:trPr>{}{}</w:tr>",
                sidebar_cell(left_content, left_width, left_background),
                sidebar_cell(right_content, right_width, right_background)
            ));
        }
        body.push_str(&format!("<w:tbl><w:tblPr><w:tblW w:w=\"{table_width}\" w:type=\"dxa\"/><w:tblLayout w:type=\"fixed\"/><w:tblBorders><w:top w:val=\"nil\"/><w:left w:val=\"nil\"/><w:bottom w:val=\"nil\"/><w:right w:val=\"nil\"/><w:insideH w:val=\"nil\"/><w:insideV w:val=\"nil\"/></w:tblBorders></w:tblPr><w:tblGrid><w:gridCol w:w=\"{left_width}\"/><w:gridCol w:w=\"{right_width}\"/></w:tblGrid>{rows}</w:tbl>"));
    } else {
        body.push_str(&main_sections.join(""));
        body.push_str(&side_sections.join(""));
    }
    // Keep adjacent tables distinct in Word-compatible importers and provide
    // the required trailing paragraph after each table, including nested ones.
    let body = word::separate_tables(&body);
    let (width, height) = match document.page_size {
        PageSize::A4 => (11906, 16838),
        PageSize::Letter => (12240, 15840),
    };
    format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><w:document xmlns:w="{WORD_NS}" xmlns:r="{REL_NS}"><w:body>{body}<w:sectPr><w:pgSz w:w="{width}" w:h="{height}"/><w:pgMar w:top="850" w:right="720" w:bottom="850" w:left="720" w:header="360" w:footer="360" w:gutter="0"/></w:sectPr></w:body></w:document>"#
    )
}

fn page_dimensions(page_size: PageSize) -> (u32, u32) {
    match page_size {
        PageSize::A4 => (11906, 16838),
        PageSize::Letter => (12240, 15840),
    }
}

#[allow(clippy::too_many_arguments)]
fn render_docx_identity(
    document: &VisualDocument,
    title: &str,
    subtitle: Option<&str>,
    eyebrow: Option<&str>,
    contact: Option<&str>,
    surface: &str,
    dark: &str,
    accent: &str,
    width: u32,
    links: &mut Vec<(String, String)>,
) -> String {
    if document.template_id == "modern_business" {
        return modern_business_identity(
            title, subtitle, eyebrow, contact, surface, accent, width, links,
        );
    }
    if document.template_id == "executive_leadership" {
        let mut content = paragraph_colored(title, "Title", "FFFFFF");
        for (value, color) in [
            (subtitle, "FFFFFF"),
            (eyebrow, "D4E0E6"),
            (contact, "D4E0E6"),
        ] {
            if let Some(value) = value.filter(|value| !value.is_empty()) {
                content.push_str(&paragraph_colored(value, "Metadata", color));
            }
        }
        let width = page_dimensions(document.page_size).0 - 1440;
        return format!(
            "<w:tbl><w:tblPr><w:tblW w:w=\"{width}\" w:type=\"dxa\"/><w:tblBorders><w:bottom w:val=\"single\" w:sz=\"20\" w:color=\"{accent}\"/></w:tblBorders></w:tblPr><w:tr><w:tc><w:tcPr><w:tcW w:w=\"{width}\" w:type=\"dxa\"/><w:tcMar><w:top w:w=\"240\" w:type=\"dxa\"/><w:bottom w:w=\"220\" w:type=\"dxa\"/><w:start w:w=\"280\" w:type=\"dxa\"/><w:end w:w=\"280\" w:type=\"dxa\"/></w:tcMar><w:shd w:val=\"clear\" w:fill=\"{dark}\"/></w:tcPr>{content}</w:tc></w:tr></w:tbl>"
        );
    }
    let mut content = paragraph(title, "Title", "");
    for (value, style) in [(subtitle, "Subtitle"), (eyebrow, "Metadata")] {
        if let Some(value) = value.filter(|value| !value.is_empty()) {
            content.push_str(&paragraph_with_links(value, style, links));
        }
    }
    if let Some(value) = contact.filter(|value| !value.is_empty()) {
        content.push_str(&paragraph_with_links(value, "Metadata", links));
    }

    match document.template_id.as_str() {
        "modern" | "creative_design" | "two_column_sidebar" => {
            let background = if document.template_id == "creative_design" {
                "F3EFF9"
            } else {
                surface
            };
            let border =
                if document.template_id == "modern" || document.template_id == "creative_design" {
                    format!("<w:bottom w:val=\"single\" w:sz=\"20\" w:color=\"{accent}\"/>")
                } else {
                    format!("<w:top w:val=\"single\" w:sz=\"20\" w:color=\"{accent}\"/>")
                };
            format!(
                "<w:tbl><w:tblPr><w:tblW w:w=\"{width}\" w:type=\"dxa\"/><w:tblBorders>{border}<w:left w:val=\"nil\"/><w:right w:val=\"nil\"/><w:insideH w:val=\"nil\"/><w:insideV w:val=\"nil\"/></w:tblBorders></w:tblPr><w:tr><w:tc><w:tcPr><w:tcW w:w=\"{width}\" w:type=\"dxa\"/><w:tcMar><w:top w:w=\"240\" w:type=\"dxa\"/><w:bottom w:w=\"220\" w:type=\"dxa\"/><w:start w:w=\"280\" w:type=\"dxa\"/><w:end w:w=\"280\" w:type=\"dxa\"/></w:tcMar><w:shd w:val=\"clear\" w:fill=\"{background}\"/></w:tcPr>{content}</w:tc></w:tr></w:tbl>"
            )
        }
        "creative_minimal" => {
            content.push_str(&format!("<w:p><w:pPr><w:pBdr><w:bottom w:val=\"single\" w:sz=\"18\" w:space=\"4\" w:color=\"{accent}\"/></w:pBdr></w:pPr><w:r><w:t/></w:r></w:p>"));
            content
        }
        _ => {
            content.push_str(&format!("<w:p><w:pPr><w:pBdr><w:bottom w:val=\"single\" w:sz=\"10\" w:space=\"5\" w:color=\"{accent}\"/></w:pBdr></w:pPr><w:r><w:t/></w:r></w:p>"));
            content
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn render_docx_section(
    document: &VisualDocument,
    kind: Option<resume_domain::SectionKind>,
    title: &str,
    entries: &[resume_domain::VisualEntry],
    sidebar: bool,
    panel: bool,
    surface: &str,
    accent: &str,
    table_width: u32,
    links: &mut Vec<(String, String)>,
) -> String {
    if sidebar {
        let mut output = colored_section_heading(title, "FFFFFF", "FFFFFF");
        for entry in entries {
            output.push_str(&paragraph_colored(&entry.heading, "Heading2", "FFFFFF"));
            if !entry.metadata.is_empty() {
                output.push_str(&paragraph_colored(
                    &entry.metadata.join(" · "),
                    "Metadata",
                    "D4E0E6",
                ));
            }
            if !entry.body.trim().is_empty() {
                output.push_str(&paragraph_colored(&entry.body, "Normal", "FFFFFF"));
            }
        }
        return output;
    }
    if kind == Some(resume_domain::SectionKind::Skill) {
        match document.template_id.as_str() {
            "modern_business" => {
                return format!(
                    "{}{}",
                    section_heading(title, accent, "D3DDE5"),
                    modern_business_skill_table(entries, accent, table_width, links)
                );
            }
            "creative_design" | "creative_minimal" | "classic" | "compact" => {
                return format!(
                    "{}{}",
                    section_heading(title, accent, "D8E0E8"),
                    skill_card_table(entries, accent, surface, table_width, links)
                );
            }
            "executive_leadership" => {
                return format!(
                    "{}{}",
                    section_heading(title, accent, "D8E0E8"),
                    executive_skill_table(entries, accent, table_width, links)
                );
            }
            _ => {}
        }
    }
    let mut output = if panel {
        paragraph(title, "Heading1", "")
    } else {
        section_heading(title, accent, "D8E0E8")
    };
    for entry in entries {
        if kind != Some(resume_domain::SectionKind::Summary) {
            output.push_str(&paragraph(&entry.heading, "Heading2", ""));
        }
        if !entry.metadata.is_empty() {
            let metadata = entry.metadata.join(" · ");
            output.push_str(&paragraph_with_links(&metadata, "Metadata", links));
        }
        if !entry.body.trim().is_empty() {
            let body = if kind == Some(resume_domain::SectionKind::Summary) {
                format!("<w:p><w:pPr><w:spacing w:after=\"100\"/><w:ind w:left=\"160\"/></w:pPr>{}</w:p>", text_runs_with_links(&entry.body, links))
            } else {
                paragraph_with_links(&entry.body, "Normal", links)
            };
            output.push_str(&body);
        }
    }
    if panel {
        let background = panel_background(&document.template_id, surface);
        // A whole-section table can be moved to the next page by Writer.
        // Paragraph shading and matching borders retain the panel while allowing
        // the section to flow naturally across page boundaries.
        output.replace("<w:pPr>", &format!("<w:pPr><w:shd w:val=\"clear\" w:fill=\"{background}\"/><w:pBdr><w:left w:val=\"single\" w:sz=\"16\" w:space=\"8\" w:color=\"{accent}\"/></w:pBdr><w:ind w:left=\"200\" w:right=\"160\"/>"))
    } else {
        output
    }
}

fn panel_background<'a>(template_id: &str, fallback: &'a str) -> &'a str {
    match template_id {
        "modern" => "F5F8FA",
        "creative_design" => "F3EFF9",
        "executive_leadership" => "F3F6F8",
        "creative_minimal" => "FBF5ED",
        "modern_business" => "F7F8F8",
        _ => fallback,
    }
}

fn section_heading(title: &str, accent: &str, rule: &str) -> String {
    format!("<w:p><w:pPr><w:pStyle w:val=\"Heading1\"/><w:pBdr><w:bottom w:val=\"single\" w:sz=\"5\" w:space=\"4\" w:color=\"{rule}\"/></w:pBdr><w:spacing w:before=\"240\" w:after=\"100\"/></w:pPr><w:r><w:rPr><w:color w:val=\"{accent}\"/></w:rPr>{}</w:r></w:p>", text_runs(title))
}

fn colored_section_heading(title: &str, color: &str, rule: &str) -> String {
    format!("<w:p><w:pPr><w:pStyle w:val=\"Heading1\"/><w:pBdr><w:bottom w:val=\"single\" w:sz=\"5\" w:space=\"4\" w:color=\"{rule}\"/></w:pBdr><w:spacing w:before=\"220\" w:after=\"90\"/></w:pPr><w:r><w:rPr><w:color w:val=\"{color}\"/></w:rPr>{}</w:r></w:p>", text_runs(title))
}

fn paragraph_colored(text: &str, style: &str, color: &str) -> String {
    format!("<w:p><w:pPr><w:pStyle w:val=\"{style}\"/></w:pPr><w:r><w:rPr><w:color w:val=\"{color}\"/></w:rPr>{}</w:r></w:p>", text_runs(text))
}

fn skill_card_table(
    entries: &[resume_domain::VisualEntry],
    accent: &str,
    surface: &str,
    width: u32,
    links: &mut Vec<(String, String)>,
) -> String {
    let mut rows = String::new();
    for pair in entries.chunks(2) {
        rows.push_str("<w:tr>");
        for entry in pair {
            let mut value = paragraph(&entry.heading, "Heading2", "");
            if !entry.metadata.is_empty() {
                value.push_str(&paragraph_with_links(
                    &entry.metadata.join(" · "),
                    "Metadata",
                    links,
                ));
            }
            if !entry.body.trim().is_empty() {
                value.push_str(&paragraph_with_links(&entry.body, "Normal", links));
            }
            rows.push_str(&format!("<w:tc><w:tcPr><w:tcMar><w:top w:w=\"90\" w:type=\"dxa\"/><w:bottom w:w=\"90\" w:type=\"dxa\"/><w:start w:w=\"130\" w:type=\"dxa\"/><w:end w:w=\"130\" w:type=\"dxa\"/></w:tcMar><w:shd w:val=\"clear\" w:fill=\"{surface}\"/><w:tcBorders><w:left w:val=\"single\" w:sz=\"10\" w:color=\"{accent}\"/></w:tcBorders></w:tcPr>{value}</w:tc>"));
        }
        if pair.len() == 1 {
            rows.push_str("<w:tc><w:tcPr><w:tcBorders><w:left w:val=\"nil\"/></w:tcBorders></w:tcPr><w:p/></w:tc>");
        }
        rows.push_str("</w:tr>");
    }
    let col = width / 2;
    format!("<w:tbl><w:tblPr><w:tblW w:w=\"{width}\" w:type=\"dxa\"/><w:tblLayout w:type=\"fixed\"/><w:tblCellSpacing w:w=\"90\" w:type=\"dxa\"/><w:tblBorders><w:top w:val=\"nil\"/><w:left w:val=\"nil\"/><w:bottom w:val=\"nil\"/><w:right w:val=\"nil\"/><w:insideH w:val=\"nil\"/><w:insideV w:val=\"nil\"/></w:tblBorders></w:tblPr><w:tblGrid><w:gridCol w:w=\"{col}\"/><w:gridCol w:w=\"{col}\"/></w:tblGrid>{rows}</w:tbl>")
}

fn executive_skill_table(
    entries: &[resume_domain::VisualEntry],
    accent: &str,
    width: u32,
    links: &mut Vec<(String, String)>,
) -> String {
    let mut rows = String::new();
    let name_width = width * 32 / 100;
    let body_width = width * 52 / 100;
    let level_width = width - name_width - body_width;
    for entry in entries {
        let metadata = entry.metadata.join(" · ");
        rows.push_str(&format!("<w:tr><w:tc><w:tcPr><w:tcW w:w=\"{name_width}\" w:type=\"dxa\"/></w:tcPr>{}</w:tc><w:tc><w:tcPr><w:tcW w:w=\"{body_width}\" w:type=\"dxa\"/></w:tcPr>{}</w:tc><w:tc><w:tcPr><w:tcW w:w=\"{level_width}\" w:type=\"dxa\"/></w:tcPr>{}</w:tc></w:tr>", paragraph(&entry.heading, "Heading2", &format!("<w:color w:val=\"{accent}\"/>")), paragraph_with_links(&entry.body, "Normal", links), paragraph_with_links_aligned(&metadata, "Metadata", "right", links)));
    }
    format!("<w:tbl><w:tblPr><w:tblW w:w=\"{width}\" w:type=\"dxa\"/><w:tblLayout w:type=\"fixed\"/><w:tblBorders><w:bottom w:val=\"single\" w:sz=\"4\" w:color=\"E2E7EC\"/></w:tblBorders></w:tblPr><w:tblGrid><w:gridCol w:w=\"{name_width}\"/><w:gridCol w:w=\"{body_width}\"/><w:gridCol w:w=\"{level_width}\"/></w:tblGrid>{rows}</w:tbl>")
}

fn paragraph(text: &str, style: &str, extra_run_properties: &str) -> String {
    word::paragraph(style, "", &word::run(text, extra_run_properties))
}

fn paragraph_with_links(text: &str, style: &str, links: &mut Vec<(String, String)>) -> String {
    word::paragraph(style, "", &text_runs_with_links(text, links))
}

#[allow(clippy::too_many_arguments)]
fn modern_business_identity(
    title: &str,
    subtitle: Option<&str>,
    eyebrow: Option<&str>,
    contact: Option<&str>,
    surface: &str,
    accent: &str,
    width: u32,
    links: &mut Vec<(String, String)>,
) -> String {
    let mut primary = paragraph(title, "Title", "");
    if let Some(value) = subtitle.filter(|value| !value.is_empty()) {
        primary.push_str(&paragraph(value, "Subtitle", ""));
    }
    if let Some(value) = eyebrow.filter(|value| !value.is_empty()) {
        primary.push_str(&paragraph_with_links(value, "Metadata", links));
    }
    let contact = contact
        .filter(|value| !value.is_empty())
        .map(|value| paragraph_with_links_aligned(value, "Metadata", "right", links))
        .unwrap_or_else(|| "<w:p/>".to_string());
    let left = width * 69 / 100;
    let right = width - left;
    format!(
        r#"<w:tbl><w:tblPr><w:tblW w:w="{width}" w:type="dxa"/><w:tblLayout w:type="fixed"/><w:tblBorders><w:top w:val="single" w:sz="20" w:space="0" w:color="{accent}"/><w:left w:val="nil"/><w:bottom w:val="nil"/><w:right w:val="nil"/><w:insideH w:val="nil"/><w:insideV w:val="nil"/></w:tblBorders></w:tblPr><w:tblGrid><w:gridCol w:w="{left}"/><w:gridCol w:w="{right}"/></w:tblGrid><w:tr><w:trPr><w:cantSplit/></w:trPr><w:tc><w:tcPr><w:tcW w:w="{left}" w:type="dxa"/><w:tcMar><w:top w:w="140" w:type="dxa"/><w:bottom w:w="140" w:type="dxa"/><w:start w:w="180" w:type="dxa"/><w:end w:w="100" w:type="dxa"/></w:tcMar><w:shd w:val="clear" w:fill="{surface}"/></w:tcPr>{primary}</w:tc><w:tc><w:tcPr><w:tcW w:w="{right}" w:type="dxa"/><w:tcMar><w:top w:w="140" w:type="dxa"/><w:bottom w:w="140" w:type="dxa"/><w:start w:w="100" w:type="dxa"/><w:end w:w="180" w:type="dxa"/></w:tcMar><w:shd w:val="clear" w:fill="{surface}"/></w:tcPr>{contact}</w:tc></w:tr></w:tbl>"#
    )
}

fn paragraph_with_links_aligned(
    text: &str,
    style: &str,
    alignment: &str,
    links: &mut Vec<(String, String)>,
) -> String {
    let runs = text_runs_with_links(text, links);
    format!("<w:p><w:pPr><w:pStyle w:val=\"{style}\"/><w:jc w:val=\"{alignment}\"/></w:pPr>{runs}</w:p>")
}

fn modern_business_skill_table(
    entries: &[resume_domain::VisualEntry],
    accent: &str,
    width: u32,
    links: &mut Vec<(String, String)>,
) -> String {
    let columns = [width / 3, width / 3, width - 2 * (width / 3)];
    let properties = format!("<w:shd w:val=\"clear\" w:fill=\"F3F6F8\"/><w:tcBorders><w:top w:val=\"single\" w:sz=\"14\" w:color=\"{accent}\"/></w:tcBorders><w:tcMar><w:top w:w=\"100\" w:type=\"dxa\"/><w:bottom w:w=\"100\" w:type=\"dxa\"/><w:start w:w=\"120\" w:type=\"dxa\"/><w:end w:w=\"120\" w:type=\"dxa\"/></w:tcMar>");
    let rows: Vec<String> = entries
        .chunks(3)
        .map(|entries| {
            let cells: Vec<String> = columns
                .iter()
                .enumerate()
                .map(|(index, width)| {
                    if let Some(entry) = entries.get(index) {
                        let mut heading = word::run(&entry.heading, "<w:b/>");
                        if !entry.metadata.is_empty() {
                            heading.push_str("<w:r><w:tab/></w:r>");
                            heading.push_str(&word::run(
                                &entry.metadata.join(" · "),
                                "<w:sz w:val=\"16\"/><w:color w:val=\"526174\"/>",
                            ));
                        }
                        let mut content = word::paragraph(
                            "Normal",
                            &format!(
                                "<w:tabs><w:tab w:val=\"right\" w:pos=\"{}\"/></w:tabs>",
                                width.saturating_sub(240)
                            ),
                            &heading,
                        );
                        if !entry.body.is_empty() {
                            content.push_str(&paragraph_with_links(&entry.body, "Normal", links));
                        }
                        word::cell(*width, &properties, &content)
                    } else {
                        word::cell(*width, "", "<w:p/>")
                    }
                })
                .collect();
            word::row(&cells)
        })
        .collect();
    word::table(width, &columns, "<w:tblCellSpacing w:w=\"80\" w:type=\"dxa\"/><w:tblBorders><w:top w:val=\"nil\"/><w:left w:val=\"nil\"/><w:bottom w:val=\"nil\"/><w:right w:val=\"nil\"/><w:insideH w:val=\"nil\"/><w:insideV w:val=\"nil\"/></w:tblBorders>", &rows)
}

fn text_runs(text: &str) -> String {
    word::text(text)
}

fn plain_run(text: &str) -> String {
    word::run(text, "")
}

fn text_runs_with_links(text: &str, links: &mut Vec<(String, String)>) -> String {
    let mut output = String::new();
    let mut cursor = 0;
    while let Some(relative) = text[cursor..]
        .find("http://")
        .or_else(|| text[cursor..].find("https://"))
    {
        let start = cursor + relative;
        output.push_str(&plain_run(&text[cursor..start]));
        let end = text[start..]
            .find(char::is_whitespace)
            .map(|offset| start + offset)
            .unwrap_or(text.len());
        let raw = &text[start..end];
        let link_end = raw.trim_end_matches(['.', ',', ';', ')', ']', '}']).len();
        if link_end == 0 {
            output.push_str(&plain_run(&text[start..end]));
            cursor = end;
            continue;
        }
        let url = &raw[..link_end];
        let id = format!("rId{}", links.len() + 2);
        links.push((id.clone(), url.to_string()));
        output.push_str(&format!("<w:hyperlink r:id=\"{id}\"><w:r><w:rPr><w:rStyle w:val=\"Hyperlink\"/></w:rPr>{}</w:r></w:hyperlink>", text_runs(url)));
        output.push_str(&plain_run(&raw[link_end..]));
        cursor = end;
    }
    output.push_str(&plain_run(&text[cursor..]));
    output
}

fn render_relationships_xml(links: &[(String, String)]) -> String {
    let mut relationships = String::from("<Relationship Id=\"rId1\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles\" Target=\"styles.xml\"/>");
    for (id, url) in links {
        relationships.push_str(&format!("<Relationship Id=\"{}\" Type=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships/hyperlink\" Target=\"{}\" TargetMode=\"External\"/>", id, escape_xml(url)));
    }
    format!("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?><Relationships xmlns=\"http://schemas.openxmlformats.org/package/2006/relationships\">{relationships}</Relationships>")
}

fn render_styles_xml(document: &VisualDocument, tokens: &serde_json::Value) -> String {
    let font = match document.style.font_family {
        FontFamily::Serif => "Georgia",
        FontFamily::SansSerif => "Arial",
    };
    let body_size = document.style.body_size_pt.max(8) * 2;
    let heading_size =
        (document.style.body_size_pt as u16 * document.style.heading_scale_percent as u16 / 100
            * 2)
        .clamp(20, 48);
    let accent = document.style.accent_color.trim_start_matches('#');
    let after = (tokens["paragraphGapMm"].as_f64().unwrap_or(1.5) * 56.693) as u32;
    let line = (240.0
        * match document.style.density {
            resume_domain::Density::Compact => tokens["compactLineHeight"].as_f64().unwrap_or(1.25),
            resume_domain::Density::Comfortable => {
                tokens["comfortableLineHeight"].as_f64().unwrap_or(1.45)
            }
        }) as u32;
    let section_size = tokens["sectionSizePt"].as_u64().unwrap_or(13) * 2;
    format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><w:styles xmlns:w="{WORD_NS}"><w:docDefaults><w:rPrDefault><w:rPr><w:rFonts w:ascii="{font}" w:hAnsi="{font}"/><w:sz w:val="{body_size}"/><w:color w:val="263445"/></w:rPr></w:rPrDefault><w:pPrDefault><w:pPr><w:spacing w:after="{after}" w:line="{line}" w:lineRule="auto"/><w:widowControl/></w:pPr></w:pPrDefault></w:docDefaults><w:style w:type="paragraph" w:default="1" w:styleId="Normal"><w:name w:val="Normal"/></w:style><w:style w:type="paragraph" w:styleId="Title"><w:name w:val="Title"/><w:basedOn w:val="Normal"/><w:pPr><w:keepNext/><w:spacing w:before="0" w:after="80"/></w:pPr><w:rPr><w:b/><w:color w:val="{accent}"/><w:sz w:val="{heading_size}"/></w:rPr></w:style><w:style w:type="paragraph" w:styleId="Subtitle"><w:name w:val="Subtitle"/><w:basedOn w:val="Normal"/><w:pPr><w:keepNext/><w:spacing w:after="40"/></w:pPr><w:rPr><w:b/><w:color w:val="263445"/></w:rPr></w:style><w:style w:type="paragraph" w:styleId="Heading1"><w:name w:val="heading 1"/><w:basedOn w:val="Normal"/><w:next w:val="Heading2"/><w:pPr><w:keepNext/><w:outlineLvl w:val="0"/><w:spacing w:before="260" w:after="100"/></w:pPr><w:rPr><w:b/><w:color w:val="{accent}"/><w:sz w:val="{section_size}"/></w:rPr></w:style><w:style w:type="paragraph" w:styleId="Heading2"><w:name w:val="heading 2"/><w:basedOn w:val="Normal"/><w:pPr><w:keepNext/><w:spacing w:before="100" w:after="20"/></w:pPr><w:rPr><w:b/><w:color w:val="263445"/></w:rPr></w:style><w:style w:type="paragraph" w:styleId="Metadata"><w:name w:val="Metadata"/><w:basedOn w:val="Normal"/><w:pPr><w:keepNext/><w:spacing w:after="60"/></w:pPr><w:rPr><w:color w:val="5B6B7E"/><w:sz w:val="18"/></w:rPr></w:style><w:style w:type="character" w:styleId="Hyperlink"><w:name w:val="Hyperlink"/><w:rPr><w:color w:val="{accent}"/><w:u w:val="single"/></w:rPr></w:style></w:styles>"#
    )
}

fn escape_xml(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn escape_html(value: &str) -> String {
    escape_xml(value)
}
