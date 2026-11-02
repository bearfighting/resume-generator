use super::{ResumeDocument, SectionKind, SemanticEntry};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const TEMPLATE_SCHEMA_VERSION: u32 = 1;
pub const VISUAL_DOCUMENT_SCHEMA_VERSION: u32 = 3;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuiltinTemplate {
    Classic,
    Modern,
    Compact,
    CreativeDesign,
    ExecutiveLeadership,
    TwoColumnModern,
    TwoColumnSidebar,
    CreativeMinimal,
    ModernBusiness,
}

impl BuiltinTemplate {
    pub fn id(self) -> &'static str {
        match self {
            Self::Classic => "classic",
            Self::Modern => "modern",
            Self::Compact => "compact",
            Self::CreativeDesign => "creative_design",
            Self::ExecutiveLeadership => "executive_leadership",
            Self::TwoColumnModern => "two_column_modern",
            Self::TwoColumnSidebar => "two_column_sidebar",
            Self::CreativeMinimal => "creative_minimal",
            Self::ModernBusiness => "modern_business",
        }
    }

    pub fn version(self) -> &'static str {
        "1.1.0"
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PageSize {
    A4,
    Letter,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FontFamily {
    SansSerif,
    Serif,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Density {
    Comfortable,
    Compact,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Layout {
    SingleColumn,
    AccentHeader,
    CompactStack,
    TwoColumn,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemplateStyle {
    pub font_family: FontFamily,
    pub body_size_pt: u8,
    pub heading_scale_percent: u8,
    pub accent_color: String,
    pub density: Density,
    #[serde(default)]
    pub presentation: Option<PresentationStyle>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemplateDefinition {
    pub schema_version: u32,
    pub id: String,
    pub version: String,
    pub page_size: PageSize,
    pub layout: Layout,
    pub style: TemplateStyle,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct StyleOverrides {
    pub palette_id: Option<String>,
    pub heading_decoration: Option<HeadingDecoration>,
    /// Percentage multiplier, constrained to 80–140 to keep layouts predictable.
    pub font_scale_percent: Option<u8>,
    pub accent_color: Option<String>,
    pub density: Option<Density>,
    pub font_family: Option<FontFamily>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HeadingDecoration {
    Rule,
    Minimal,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PresentationStyle {
    pub surface: String,
    pub rule: String,
    pub dark: String,
    pub heading_decoration: Option<HeadingDecoration>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Palette {
    pub id: String,
    pub name: String,
    pub accent: String,
    pub surface: String,
    pub rule: String,
    pub dark: String,
}

pub fn template_palettes(template_id: &str) -> Vec<Palette> {
    let colors = match template_id {
        "modern" => [
            ("Ocean", "#176B87"),
            ("Forest", "#276247"),
            ("Slate", "#414D5D"),
            ("Indigo", "#454B87"),
        ],
        "creative_design" => [
            ("Violet", "#7054A3"),
            ("Terracotta", "#934A35"),
            ("Berry", "#8A365C"),
            ("Lagoon", "#176D70"),
        ],
        "compact" => [
            ("Ink", "#263A50"),
            ("Burgundy", "#75394A"),
            ("Evergreen", "#315844"),
            ("Graphite", "#42464D"),
        ],
        "executive_leadership" => [
            ("Midnight", "#17324D"),
            ("Charcoal", "#343E48"),
            ("Pine", "#24523F"),
            ("Walnut", "#694B35"),
        ],
        "two_column_modern" => [
            ("Teal", "#245D6C"),
            ("Sapphire", "#305B83"),
            ("Olive", "#52603A"),
            ("Plum", "#704C70"),
        ],
        "two_column_sidebar" => [
            ("Deep teal", "#1F5368"),
            ("Navy", "#304E7A"),
            ("Spruce", "#315B4A"),
            ("Aubergine", "#68466A"),
        ],
        "creative_minimal" => [
            ("Ochre", "#80532C"),
            ("Clay", "#964B40"),
            ("Sage", "#48664D"),
            ("Dusty blue", "#406880"),
        ],
        "modern_business" => [
            ("Corporate blue", "#284D70"),
            ("Petrol", "#245D61"),
            ("Steel", "#4C5564"),
            ("Merlot", "#773C50"),
        ],
        _ => [
            ("Ink blue", "#243B53"),
            ("Black", "#292D33"),
            ("Wine", "#713D4B"),
            ("Hunter", "#31563E"),
        ],
    };
    colors
        .into_iter()
        .enumerate()
        .map(|(index, (name, accent))| {
            let rgb = u32::from_str_radix(&accent[1..], 16).unwrap_or(0);
            let mix = |white: u32| {
                let channel =
                    |shift: u32| (((rgb >> shift) & 255) * (100 - white) + 255 * white) / 100;
                format!("#{:02X}{:02X}{:02X}", channel(16), channel(8), channel(0))
            };
            let dark = format!(
                "#{:02X}{:02X}{:02X}",
                ((rgb >> 16) & 255) / 2,
                ((rgb >> 8) & 255) / 2,
                (rgb & 255) / 2
            );
            Palette {
                id: format!("palette_{}", index + 1),
                name: name.into(),
                accent: accent.into(),
                surface: mix(94),
                rule: mix(65),
                dark,
            }
        })
        .collect()
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentDocument {
    pub title: String,
    pub subtitle: Option<String>,
    pub eyebrow: Option<String>,
    pub contact: Option<String>,
    pub sections: Vec<ContentSection>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentSection {
    #[serde(default)]
    pub kind: Option<SectionKind>,
    pub title: String,
    pub entries: Vec<ContentEntry>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentEntry {
    pub heading: String,
    pub metadata: Vec<String>,
    pub body: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualDocument {
    pub schema_version: u32,
    pub template_id: String,
    pub template_version: String,
    pub page_size: PageSize,
    pub layout: Layout,
    pub style: TemplateStyle,
    pub blocks: Vec<VisualBlock>,
    pub diagnostics: Vec<Diagnostic>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum VisualBlock {
    Identity {
        title: String,
        subtitle: Option<String>,
        eyebrow: Option<String>,
        contact: Option<String>,
    },
    Section {
        kind: Option<SectionKind>,
        title: String,
        entries: Vec<VisualEntry>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VisualEntry {
    pub heading: String,
    pub metadata: Vec<String>,
    pub body: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticSeverity {
    Info,
    Warning,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Diagnostic {
    pub severity: DiagnosticSeverity,
    pub code: String,
    pub message: String,
    pub path: Option<String>,
}

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum TemplateError {
    #[error("unknown built-in template: {0}")]
    UnknownTemplate(String),
    #[error("unsupported template schema version: {0}")]
    UnsupportedSchemaVersion(u32),
    #[error("unsupported semantic document schema version: {0}")]
    UnsupportedDocumentSchemaVersion(u32),
    #[error("template id and version must not be empty")]
    EmptyTemplateIdentity,
    #[error("invalid accent color; expected #RRGGBB")]
    InvalidAccentColor,
    #[error("font scale must be between 80 and 140 percent")]
    InvalidFontScale,
    #[error("document title must not be empty")]
    EmptyDocumentTitle,
    #[error("section title must not be empty")]
    EmptySectionTitle,
    #[error("entry heading must not be empty")]
    EmptyEntryHeading,
    #[error("unknown palette for selected template: {0}")]
    UnknownPalette(String),
}

pub fn builtin_template(template: BuiltinTemplate) -> TemplateDefinition {
    let (page_size, layout, style) = match template {
        BuiltinTemplate::Classic => (
            PageSize::A4,
            Layout::SingleColumn,
            TemplateStyle {
                font_family: FontFamily::Serif,
                body_size_pt: 11,
                heading_scale_percent: 145,
                presentation: None,
                accent_color: "#243B53".into(),
                density: Density::Comfortable,
            },
        ),
        BuiltinTemplate::Modern => (
            PageSize::A4,
            Layout::AccentHeader,
            TemplateStyle {
                font_family: FontFamily::SansSerif,
                body_size_pt: 10,
                heading_scale_percent: 155,
                presentation: None,
                accent_color: "#176B87".into(),
                density: Density::Comfortable,
            },
        ),
        BuiltinTemplate::Compact => (
            PageSize::Letter,
            Layout::CompactStack,
            TemplateStyle {
                font_family: FontFamily::Serif,
                body_size_pt: 11,
                heading_scale_percent: 135,
                presentation: None,
                accent_color: "#263A50".into(),
                density: Density::Comfortable,
            },
        ),
        BuiltinTemplate::CreativeDesign => (
            PageSize::A4,
            Layout::SingleColumn,
            TemplateStyle {
                font_family: FontFamily::Serif,
                body_size_pt: 10,
                heading_scale_percent: 160,
                presentation: None,
                accent_color: "#7054A3".into(),
                density: Density::Comfortable,
            },
        ),
        BuiltinTemplate::ExecutiveLeadership => (
            PageSize::A4,
            Layout::AccentHeader,
            TemplateStyle {
                font_family: FontFamily::SansSerif,
                body_size_pt: 10,
                heading_scale_percent: 150,
                presentation: None,
                accent_color: "#17324D".into(),
                density: Density::Comfortable,
            },
        ),
        BuiltinTemplate::TwoColumnModern => (
            PageSize::A4,
            Layout::TwoColumn,
            TemplateStyle {
                font_family: FontFamily::SansSerif,
                body_size_pt: 9,
                heading_scale_percent: 145,
                presentation: None,
                accent_color: "#245D6C".into(),
                density: Density::Comfortable,
            },
        ),
        BuiltinTemplate::TwoColumnSidebar => (
            PageSize::A4,
            Layout::TwoColumn,
            TemplateStyle {
                font_family: FontFamily::SansSerif,
                body_size_pt: 9,
                heading_scale_percent: 145,
                presentation: None,
                accent_color: "#1F5368".into(),
                density: Density::Comfortable,
            },
        ),
        BuiltinTemplate::CreativeMinimal => (
            PageSize::A4,
            Layout::SingleColumn,
            TemplateStyle {
                font_family: FontFamily::Serif,
                body_size_pt: 10,
                heading_scale_percent: 150,
                presentation: None,
                accent_color: "#C47D3D".into(),
                density: Density::Comfortable,
            },
        ),
        BuiltinTemplate::ModernBusiness => (
            PageSize::A4,
            Layout::AccentHeader,
            TemplateStyle {
                font_family: FontFamily::SansSerif,
                body_size_pt: 10,
                heading_scale_percent: 145,
                presentation: None,
                accent_color: "#284D70".into(),
                density: Density::Comfortable,
            },
        ),
    };

    TemplateDefinition {
        schema_version: TEMPLATE_SCHEMA_VERSION,
        id: template.id().into(),
        version: template.version().into(),
        page_size,
        layout,
        style,
    }
}

impl TemplateDefinition {
    pub fn validate(&self) -> Result<(), TemplateError> {
        if self.schema_version != TEMPLATE_SCHEMA_VERSION {
            return Err(TemplateError::UnsupportedSchemaVersion(self.schema_version));
        }
        if self.id.trim().is_empty() || self.version.trim().is_empty() {
            return Err(TemplateError::EmptyTemplateIdentity);
        }
        if !is_hex_color(&self.style.accent_color) {
            return Err(TemplateError::InvalidAccentColor);
        }
        Ok(())
    }
}

pub fn apply_template(
    document: &ContentDocument,
    template_id: &str,
    overrides: &StyleOverrides,
) -> Result<VisualDocument, TemplateError> {
    let template = match template_id {
        "classic" => builtin_template(BuiltinTemplate::Classic),
        "modern" => builtin_template(BuiltinTemplate::Modern),
        "compact" => builtin_template(BuiltinTemplate::Compact),
        "creative_design" => builtin_template(BuiltinTemplate::CreativeDesign),
        "executive_leadership" => builtin_template(BuiltinTemplate::ExecutiveLeadership),
        "two_column_modern" => builtin_template(BuiltinTemplate::TwoColumnModern),
        "two_column_sidebar" => builtin_template(BuiltinTemplate::TwoColumnSidebar),
        "creative_minimal" => builtin_template(BuiltinTemplate::CreativeMinimal),
        "modern_business" => builtin_template(BuiltinTemplate::ModernBusiness),
        other => return Err(TemplateError::UnknownTemplate(other.into())),
    };
    apply_template_definition(document, &template, overrides)
}

fn apply_template_definition(
    document: &ContentDocument,
    template: &TemplateDefinition,
    overrides: &StyleOverrides,
) -> Result<VisualDocument, TemplateError> {
    template.validate()?;
    validate_content(document)?;
    if let Some(scale) = overrides.font_scale_percent {
        if !(80..=140).contains(&scale) {
            return Err(TemplateError::InvalidFontScale);
        }
    }
    if let Some(color) = &overrides.accent_color {
        if !is_hex_color(color) {
            return Err(TemplateError::InvalidAccentColor);
        }
    }

    let mut style = template.style.clone();
    if overrides.palette_id.is_some() || overrides.heading_decoration.is_some() {
        let palette = if let Some(id) = &overrides.palette_id {
            Some(
                template_palettes(&template.id)
                    .into_iter()
                    .find(|p| &p.id == id)
                    .ok_or_else(|| TemplateError::UnknownPalette(id.clone()))?,
            )
        } else {
            None
        };
        if let Some(palette) = palette {
            style.accent_color = palette.accent.clone();
            style.presentation = Some(PresentationStyle {
                surface: palette.surface,
                rule: palette.rule,
                dark: palette.dark,
                heading_decoration: overrides.heading_decoration,
            });
        } else {
            style.presentation = Some(PresentationStyle {
                surface: String::new(),
                rule: String::new(),
                dark: String::new(),
                heading_decoration: overrides.heading_decoration,
            });
        }
    }
    if let Some(scale) = overrides.font_scale_percent {
        style.body_size_pt =
            ((u16::from(style.body_size_pt) * u16::from(scale) + 50) / 100).clamp(7, 18) as u8;
        style.heading_scale_percent = ((u16::from(style.heading_scale_percent) * u16::from(scale)
            + 50)
            / 100)
            .clamp(110, 220) as u8;
    }
    if let Some(color) = &overrides.accent_color {
        style.accent_color = color.to_ascii_uppercase();
    }
    if let Some(density) = overrides.density {
        style.density = density;
    }
    if let Some(font_family) = overrides.font_family {
        style.font_family = font_family;
    }

    let mut diagnostics = Vec::new();
    let mut blocks = vec![VisualBlock::Identity {
        title: document.title.clone(),
        subtitle: document.subtitle.clone(),
        eyebrow: document.eyebrow.clone(),
        contact: document.contact.clone(),
    }];
    for (section_index, section) in document.sections.iter().enumerate() {
        if section.entries.is_empty() {
            continue;
        }
        let entries = section
            .entries
            .iter()
            .enumerate()
            .map(|(entry_index, entry)| {
                if entry.body.chars().count() > 2000 {
                    diagnostics.push(Diagnostic {
                        severity: DiagnosticSeverity::Warning,
                        code: "long_text".into(),
                        message: "Long text may flow across pages; review the rendered result."
                            .into(),
                        path: Some(format!(
                            "sections[{section_index}].entries[{entry_index}].body"
                        )),
                    });
                }
                VisualEntry {
                    heading: entry.heading.clone(),
                    metadata: entry.metadata.clone(),
                    body: entry.body.clone(),
                }
            })
            .collect();
        blocks.push(VisualBlock::Section {
            kind: section.kind,
            title: section.title.clone(),
            entries,
        });
    }

    Ok(VisualDocument {
        schema_version: VISUAL_DOCUMENT_SCHEMA_VERSION,
        template_id: template.id.clone(),
        template_version: template.version.clone(),
        page_size: template.page_size,
        layout: template.layout,
        style,
        blocks,
        diagnostics,
    })
}

pub fn apply_template_to_resume(
    document: &ResumeDocument,
    template_id: &str,
    overrides: &StyleOverrides,
) -> Result<VisualDocument, TemplateError> {
    if document.schema_version != super::DOCUMENT_SCHEMA_VERSION {
        return Err(TemplateError::UnsupportedDocumentSchemaVersion(
            document.schema_version,
        ));
    }
    let content = ContentDocument {
        title: document.personal.full_name.clone(),
        subtitle: document.target_role.clone(),
        eyebrow: document.personal.headline.clone(),
        contact: {
            let details = [
                document.personal.location.as_deref(),
                document.personal.phone.as_deref(),
                document.personal.email.as_deref(),
                document.personal.website.as_deref(),
                document.personal.linkedin.as_deref(),
                document.personal.github.as_deref(),
            ]
            .into_iter()
            .flatten()
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .collect::<Vec<_>>();
            (!details.is_empty()).then(|| details.join(" · "))
        },
        sections: document
            .sections
            .iter()
            .map(|section| ContentSection {
                kind: Some(section.kind),
                title: section.title.clone(),
                entries: section.entries.iter().map(map_semantic_entry).collect(),
            })
            .collect(),
    };
    apply_template(&content, template_id, overrides)
}

fn map_semantic_entry(experience: &SemanticEntry) -> ContentEntry {
    ContentEntry {
        heading: experience.heading.clone(),
        metadata: experience.metadata.clone(),
        body: experience.body.clone(),
    }
}

fn validate_content(document: &ContentDocument) -> Result<(), TemplateError> {
    if document.title.trim().is_empty() {
        return Err(TemplateError::EmptyDocumentTitle);
    }
    for section in &document.sections {
        if section.title.trim().is_empty() {
            return Err(TemplateError::EmptySectionTitle);
        }
        if section
            .entries
            .iter()
            .any(|entry| entry.heading.trim().is_empty())
        {
            return Err(TemplateError::EmptyEntryHeading);
        }
    }
    Ok(())
}

fn is_hex_color(value: &str) -> bool {
    value.len() == 7
        && value.starts_with('#')
        && value[1..].bytes().all(|byte| byte.is_ascii_hexdigit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn palettes_and_decorations_compile_and_round_trip() {
        for id in [
            "classic",
            "modern",
            "compact",
            "creative_design",
            "executive_leadership",
            "two_column_modern",
            "two_column_sidebar",
            "creative_minimal",
            "modern_business",
        ] {
            let palettes = template_palettes(id);
            assert_eq!(palettes.len(), 4);
            for palette in palettes {
                for decoration in [HeadingDecoration::Rule, HeadingDecoration::Minimal] {
                    let overrides = StyleOverrides {
                        palette_id: Some(palette.id.clone()),
                        heading_decoration: Some(decoration),
                        ..StyleOverrides::default()
                    };
                    let decoded: StyleOverrides =
                        serde_json::from_str(&serde_json::to_string(&overrides).unwrap()).unwrap();
                    assert_eq!(decoded, overrides);
                    let visual = apply_template(&content_document(), id, &decoded).unwrap();
                    assert_eq!(visual.style.accent_color, palette.accent);
                    assert_eq!(
                        visual.style.presentation.unwrap().heading_decoration,
                        Some(decoration)
                    );
                }
            }
        }
        let invalid = StyleOverrides {
            palette_id: Some("missing".into()),
            ..StyleOverrides::default()
        };
        assert!(matches!(
            apply_template(&content_document(), "modern", &invalid),
            Err(TemplateError::UnknownPalette(_))
        ));
        let legacy: StyleOverrides = serde_json::from_str("{}").unwrap();
        assert!(apply_template(&content_document(), "modern", &legacy)
            .unwrap()
            .style
            .presentation
            .is_none());
    }

    fn content_document() -> ContentDocument {
        ContentDocument {
            title: "Alex Morgan".into(),
            subtitle: Some("Senior Product Engineer".into()),
            eyebrow: Some("Product Engineer".into()),
            contact: Some("alex@example.test".into()),
            sections: vec![ContentSection {
                kind: None,
                title: "Experience".into(),
                entries: vec![ContentEntry {
                    heading: "Engineer — Northstar".into(),
                    metadata: vec!["2020–2024".into()],
                    body: "Built private-by-default desktop software.".into(),
                }],
            }],
        }
    }

    #[test]
    fn all_built_in_templates_compile_without_losing_content() {
        let document = content_document();
        let mut outputs = Vec::new();
        for template in [
            BuiltinTemplate::Classic,
            BuiltinTemplate::Modern,
            BuiltinTemplate::Compact,
        ] {
            let definition = builtin_template(template);
            definition.validate().expect("built-in template is valid");
            let visual = apply_template(&document, template.id(), &StyleOverrides::default())
                .expect("built-in template compiles");
            assert_eq!(visual.template_id, template.id());
            assert_eq!(visual.template_version, "1.1.0");
            assert_eq!(visual.blocks.len(), 2);
            assert_eq!(visual.diagnostics, Vec::new());
            outputs.push(visual);
        }

        assert_ne!(outputs[0].layout, outputs[1].layout);
        assert_ne!(outputs[1].style, outputs[2].style);
        let expected_identity = VisualBlock::Identity {
            title: "Alex Morgan".into(),
            subtitle: Some("Senior Product Engineer".into()),
            eyebrow: Some("Product Engineer".into()),
            contact: Some("alex@example.test".into()),
        };
        let expected_section = VisualBlock::Section {
            kind: None,
            title: "Experience".into(),
            entries: vec![VisualEntry {
                heading: "Engineer — Northstar".into(),
                metadata: vec!["2020–2024".into()],
                body: "Built private-by-default desktop software.".into(),
            }],
        };
        for output in outputs {
            assert_eq!(output.blocks[0], expected_identity);
            assert_eq!(output.blocks[1], expected_section);
        }
    }

    #[test]
    fn style_overrides_are_constrained_and_report_long_text() {
        let mut document = content_document();
        document.sections[0].entries[0].body = "经验🙂".repeat(1001);
        let output = apply_template(
            &document,
            "modern",
            &StyleOverrides {
                font_scale_percent: Some(120),
                accent_color: Some("#aa22CC".into()),
                density: Some(Density::Compact),
                font_family: Some(FontFamily::Serif),
                ..StyleOverrides::default()
            },
        )
        .expect("supported overrides compile");

        assert_eq!(output.style.accent_color, "#AA22CC");
        assert_eq!(output.style.density, Density::Compact);
        assert_eq!(output.style.font_family, FontFamily::Serif);
        assert_eq!(output.diagnostics.len(), 1);
        assert_eq!(output.diagnostics[0].code, "long_text");
        assert_eq!(
            output.blocks[1],
            VisualBlock::Section {
                kind: None,
                title: "Experience".into(),
                entries: vec![VisualEntry {
                    heading: "Engineer — Northstar".into(),
                    metadata: vec!["2020–2024".into()],
                    body: "经验🙂".repeat(1001),
                }],
            }
        );
        assert!(matches!(
            apply_template(
                &content_document(),
                "modern",
                &StyleOverrides {
                    font_scale_percent: Some(150),
                    ..StyleOverrides::default()
                }
            ),
            Err(TemplateError::InvalidFontScale)
        ));
    }

    #[test]
    fn template_validation_rejects_unknown_templates_and_invalid_colors() {
        assert_eq!(
            apply_template(&content_document(), "unknown", &StyleOverrides::default()),
            Err(TemplateError::UnknownTemplate("unknown".into()))
        );
        assert_eq!(
            apply_template(
                &content_document(),
                "classic",
                &StyleOverrides {
                    accent_color: Some("blue".into()),
                    ..StyleOverrides::default()
                }
            ),
            Err(TemplateError::InvalidAccentColor)
        );
    }

    #[test]
    fn template_compiles_a_semantic_resume_document() {
        let semantic = ResumeDocument {
            schema_version: super::super::DOCUMENT_SCHEMA_VERSION,
            title: "Product Engineer Resume".into(),
            target_role: Some("Senior Product Engineer".into()),
            personal: super::super::PersonalInfo {
                full_name: "Alex Morgan".into(),
                headline: Some("Product Engineer".into()),
                email: Some("alex@example.test".into()),
                phone: None,
                location: None,
                website: None,
                linkedin: None,
                github: None,
                professional_summary: None,
            },
            sections: vec![super::super::SemanticSection {
                kind: super::super::SectionKind::Experience,
                title: "Experience".into(),
                entries: vec![SemanticEntry {
                    source_id: "exp-1".into(),
                    heading: "Engineer — Northstar".into(),
                    metadata: vec!["2020–2024".into()],
                    body: "Built local software".into(),
                }],
            }],
        };
        let visual = apply_template_to_resume(&semantic, "classic", &StyleOverrides::default())
            .expect("resume template compile succeeds");
        assert_eq!(
            visual.blocks[0],
            VisualBlock::Identity {
                title: "Alex Morgan".into(),
                subtitle: Some("Senior Product Engineer".into()),
                eyebrow: Some("Product Engineer".into()),
                contact: Some("alex@example.test".into()),
            }
        );
        assert_eq!(visual.blocks.len(), 2);
    }

    #[test]
    fn template_json_round_trip_preserves_the_schema() {
        let template = builtin_template(BuiltinTemplate::Modern);
        let json = serde_json::to_string(&template).expect("template serializes");
        assert_eq!(
            serde_json::from_str::<TemplateDefinition>(&json).expect("template deserializes"),
            template
        );
    }
}
