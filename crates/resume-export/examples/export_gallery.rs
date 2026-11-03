use resume_domain::{
    apply_template, ContentDocument, ContentEntry, ContentSection, PageSize, SectionKind,
    StyleOverrides,
};
use std::{fs, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let directory = PathBuf::from(
        std::env::args()
            .nth(1)
            .unwrap_or_else(|| "/tmp/resume-export-gallery".into()),
    );
    fs::create_dir_all(&directory)?;
    let entry = |heading: &str, body: &str| ContentEntry {
        heading: heading.into(),
        metadata: vec!["2021–Present · Advanced".into()],
        body: body.into(),
    };
    let templates = [
        "classic",
        "modern",
        "compact",
        "creative_design",
        "executive_leadership",
        "two_column_modern",
        "two_column_sidebar",
        "creative_minimal",
        "modern_business",
    ];
    for template in templates {
        for (paper, size) in [("a4", PageSize::A4), ("letter", PageSize::Letter)] {
            for sparse in [false, true] {
                let source = ContentDocument {
                    title: "Alex Chen 陈明 — Élodie".into(),
                    subtitle: Some("Principal Software Engineer".into()),
                    eyebrow: Some("Engineering Leader · Platform & Developer Experience".into()),
                    contact: Some("Toronto · alex@example.test · https://example.test/alex".into()),
                    sections: if sparse {
                        vec![ContentSection {
                            kind: Some(SectionKind::Skill),
                            title: "Skills".into(),
                            entries: vec![entry("Rust", "Systems engineering")],
                        }]
                    } else {
                        vec![
                        ContentSection {kind: Some(SectionKind::Experience), title: "Experience".into(), entries: (0..8).map(|i| entry(&format!("Principal Engineer {i}"), &"Built inclusive platform tools for global teams. ".repeat(8))).collect()},
                        ContentSection {kind: Some(SectionKind::Skill), title: "Skills".into(), entries: (0..7).map(|i| entry(&format!("Skill {i}"), "Application and platform tooling")).collect()},
                        ContentSection {kind: Some(SectionKind::Certification), title: "Certifications".into(), entries: vec![entry("Final certification", "Verify the final paragraph survives export. https://example.test/credential")]},
                    ]
                    },
                };
                let mut document = apply_template(&source, template, &StyleOverrides::default())?;
                document.page_size = size;
                let name = format!(
                    "{template}-{paper}-{}.docx",
                    if sparse { "sparse" } else { "long" }
                );
                resume_export::export_docx_with_mode(
                    &directory.join(&name),
                    &document,
                    resume_export::DocxMode::Design,
                )?;
                let standard = directory.join("standard");
                fs::create_dir_all(&standard)?;
                resume_export::export_docx(&standard.join(name), &document)?;
            }
        }
    }
    Ok(())
}
