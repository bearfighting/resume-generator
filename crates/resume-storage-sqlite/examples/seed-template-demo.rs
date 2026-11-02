use resume_domain::{
    Award, CareerProfile, Certification, DraftEntry, DraftSection, Education, Experience, Interest,
    Language, License, Membership, PersonalInfo, ProfessionalReference, Project, Publication,
    ResumeDraft, SectionKind, Skill, StyleOverrides, VolunteerExperience, DRAFT_SCHEMA_VERSION,
    PROFILE_SCHEMA_VERSION,
};
use resume_storage_sqlite::SqliteResumeRepository;
use std::{error::Error, path::PathBuf};

const PROFILE_ID: &str = "profile-template-demo";
const DRAFT_ID: &str = "resume-template-demo";
const UPDATED_AT: &str = "2026-11-02T16:00:00Z";

fn main() -> Result<(), Box<dyn Error>> {
    let database_path = std::env::args_os().nth(1).map(PathBuf::from).ok_or(
        "usage: cargo run -p resume-storage-sqlite --example seed-template-demo -- <sqlite-path>",
    )?;
    let mut repository = SqliteResumeRepository::open(database_path)?;
    let profile = demo_profile();
    let draft = demo_draft();

    repository.save_workspace(&profile, &draft)?;
    println!(
        "Saved sample profile {PROFILE_ID} and resume {DRAFT_ID}; running this command again updates the same records."
    );
    Ok(())
}

fn demo_profile() -> CareerProfile {
    CareerProfile {
        id: PROFILE_ID.into(),
        schema_version: PROFILE_SCHEMA_VERSION,
        personal: PersonalInfo {
            full_name: "Alex Chen".into(),
            headline: Some("Engineering Leader · Platform & Developer Experience".into()),
            email: Some("alex.chen@example.test".into()),
            phone: Some("+1 416 555 0142".into()),
            location: Some("Toronto, ON".into()),
            website: Some("https://alexchen.example.test".into()),
            linkedin: Some("https://linkedin.example.test/in/alexchen".into()),
            github: Some("https://github.example.test/alexchen".into()),
            professional_summary: Some("Platform engineering leader with 12 years of experience building reliable developer tools and distributed systems. Known for turning operational pain points into measurable improvements in delivery speed, service health, and team autonomy.".into()),
        },
        experiences: vec![
            Experience {
                id: "template-demo-experience-northstar".into(),
                role: "Principal Software Engineer".into(),
                organization: "Northstar Systems".into(),
                period: "2021–Present".into(),
                summary: "Led a five-person platform team serving 42 product and infrastructure teams. Reduced deployment failures by 38% and median recovery time from 28 to 11 minutes. Introduced service ownership and accessibility standards adopted across the engineering organization.".into(),
            },
            Experience {
                id: "template-demo-experience-cedar".into(),
                role: "Senior Software Engineer".into(),
                organization: "Cedar Labs".into(),
                period: "2017–2021".into(),
                summary: "Designed APIs and event-processing services used by 1.2 million monthly customers. Improved account setup completion by 24% through a simpler onboarding workflow. Mentored four engineers and coordinated releases with product, research, and operations.".into(),
            },
            Experience {
                id: "template-demo-experience-fieldnote".into(),
                role: "Software Engineer".into(),
                organization: "Fieldnote Studio".into(),
                period: "2014–2017".into(),
                summary: "Built internal tools and customer-facing services for education and public-sector clients. Automated recurring data imports, reducing manual operations work by approximately 12 hours per week.".into(),
            },
        ],
        education: vec![Education {
            id: "template-demo-education-waterloo".into(),
            institution: "University of Waterloo".into(),
            degree: Some("BSc".into()),
            field_of_study: Some("Computer Science".into()),
            period: Some("2010–2014".into()),
            summary: "Focus on distributed systems, databases, and human-computer interaction.".into(),
        }],
        projects: vec![
            Project {
                id: "template-demo-project-observability".into(),
                name: "Open Observability Workbench".into(),
                role: Some("Project Lead".into()),
                period: Some("2024".into()),
                url: Some("https://example.test/projects/open-observability-workbench".into()),
                summary: "Built a local-first diagnostic workflow that brings logs, traces, and deployment events together. Documented the design and onboarding path for teams adopting the tool.".into(),
            },
            Project {
                id: "template-demo-project-readiness".into(),
                name: "Deployment Readiness Scorecard".into(),
                role: Some("Technical Lead".into()),
                period: Some("2022–2023".into()),
                url: Some("https://example.test/projects/deployment-readiness".into()),
                summary: "Created a service health checklist and rollout dashboard that helped teams identify risky releases before production deployment.".into(),
            },
        ],
        skills: vec![
            Skill { id: "template-demo-skill-rust".into(), name: "Rust".into(), level: Some("Advanced".into()), summary: "Application and platform tooling".into() },
            Skill { id: "template-demo-skill-typescript".into(), name: "TypeScript".into(), level: Some("Advanced".into()), summary: "Web applications and developer experience".into() },
            Skill { id: "template-demo-skill-systems".into(), name: "Distributed systems".into(), level: Some("Advanced".into()), summary: "Service design, event processing, and reliability".into() },
            Skill { id: "template-demo-skill-postgres".into(), name: "PostgreSQL".into(), level: Some("Advanced".into()), summary: "Data modeling and query performance".into() },
            Skill { id: "template-demo-skill-kubernetes".into(), name: "Kubernetes".into(), level: Some("Working".into()), summary: "Deployment platforms and service operations".into() },
            Skill { id: "template-demo-skill-observability".into(), name: "Observability".into(), level: Some("Advanced".into()), summary: "Metrics, traces, logs, and incident response".into() },
            Skill { id: "template-demo-skill-accessibility".into(), name: "Accessibility".into(), level: Some("Working".into()), summary: "Inclusive product and platform practices".into() },
        ],
        certifications: vec![
            Certification {
                id: "template-demo-cert-kubernetes".into(),
                name: "Certified Kubernetes Administrator".into(),
                issuer: "The Linux Foundation".into(),
                period: Some("2023".into()),
                credential_url: Some("https://example.test/credentials/cka".into()),
            },
            Certification {
                id: "template-demo-cert-cloud".into(),
                name: "Professional Cloud Architect".into(),
                issuer: "Google Cloud".into(),
                period: Some("2022".into()),
                credential_url: Some("https://example.test/credentials/cloud-architect".into()),
            },
        ],
        languages: vec![
            Language { id: "template-demo-language-english".into(), name: "English".into(), proficiency: Some("Fluent".into()), certification: Some("IELTS Academic 8.0".into()) },
            Language { id: "template-demo-language-french".into(), name: "French".into(), proficiency: Some("Professional working proficiency".into()), certification: Some("DELF B2".into()) },
        ],
        awards: vec![Award {
            id: "template-demo-award-platform-impact".into(),
            name: "Platform Impact Award".into(),
            issuer: Some("Northstar Systems Engineering".into()),
            period: Some("2024".into()),
            summary: "Recognized for a reliability program that reduced deployment failures across 42 teams.".into(),
            url: Some("https://example.test/awards/platform-impact-2024".into()),
        }],
        volunteering: vec![VolunteerExperience {
            id: "template-demo-volunteer-codebridge".into(),
            role: "Technical Mentor".into(),
            organization: "CodeBridge Toronto".into(),
            period: Some("2022–Present".into()),
            summary: "Mentor early-career developers through portfolio reviews and monthly project clinics.".into(),
        }],
        publications: vec![Publication {
            id: "template-demo-publication-operational-readiness".into(),
            title: "Making Operational Readiness a Product Practice".into(),
            venue: Some("Canadian Software Engineering Summit".into()),
            period: Some("2023".into()),
            url: Some("https://example.test/talks/operational-readiness".into()),
            summary: "Talk on service ownership, progressive delivery, and sustainable incident response.".into(),
        }],
        memberships: vec![Membership {
            id: "template-demo-membership-acm".into(),
            organization: "Association for Computing Machinery".into(),
            role: Some("Professional Member".into()),
            period: Some("2018–Present".into()),
        }],
        licenses: vec![License {
            id: "template-demo-license-cloud".into(),
            name: "Cloud Solutions Architect License".into(),
            issuer: Some("Example Cloud Institute".into()),
            period: Some("2025–2028".into()),
            credential_url: Some("https://example.test/licenses/cloud-architect".into()),
        }],
        references: vec![ProfessionalReference {
            id: "template-demo-reference-jordan".into(),
            name: "Jordan Lee".into(),
            role: Some("Director of Engineering".into()),
            organization: Some("Northstar Systems".into()),
            email: Some("jordan.lee@example.test".into()),
            phone: Some("+1 416 555 0168".into()),
        }],
        interests: vec![
            Interest { id: "template-demo-interest-open-source".into(), name: "Open-source mentoring".into() },
            Interest { id: "template-demo-interest-trails".into(), name: "Long-distance hiking".into() },
        ],
        updated_at: UPDATED_AT.into(),
    }
}

fn demo_draft() -> ResumeDraft {
    ResumeDraft {
        id: DRAFT_ID.into(),
        schema_version: DRAFT_SCHEMA_VERSION,
        profile_id: PROFILE_ID.into(),
        title: "Platform Engineering Leadership".into(),
        target_role: Some("Principal Software Engineer".into()),
        summary_included: true,
        summary_override: None,
        sections: vec![
            section(
                "template-demo-section-experience",
                SectionKind::Experience,
                "Experience",
                &[
                    "template-demo-experience-northstar",
                    "template-demo-experience-cedar",
                    "template-demo-experience-fieldnote",
                ],
            ),
            section(
                "template-demo-section-projects",
                SectionKind::Project,
                "Selected Projects",
                &[
                    "template-demo-project-observability",
                    "template-demo-project-readiness",
                ],
            ),
            section(
                "template-demo-section-education",
                SectionKind::Education,
                "Education",
                &["template-demo-education-waterloo"],
            ),
            section(
                "template-demo-section-skills",
                SectionKind::Skill,
                "Skills",
                &[
                    "template-demo-skill-rust",
                    "template-demo-skill-typescript",
                    "template-demo-skill-systems",
                    "template-demo-skill-postgres",
                    "template-demo-skill-kubernetes",
                    "template-demo-skill-observability",
                    "template-demo-skill-accessibility",
                ],
            ),
            section(
                "template-demo-section-certifications",
                SectionKind::Certification,
                "Certifications",
                &["template-demo-cert-kubernetes", "template-demo-cert-cloud"],
            ),
            section(
                "template-demo-section-languages",
                SectionKind::Language,
                "Languages",
                &[
                    "template-demo-language-english",
                    "template-demo-language-french",
                ],
            ),
            section(
                "template-demo-section-awards",
                SectionKind::Award,
                "Awards",
                &["template-demo-award-platform-impact"],
            ),
            section(
                "template-demo-section-volunteering",
                SectionKind::Volunteering,
                "Volunteer Experience",
                &["template-demo-volunteer-codebridge"],
            ),
            section(
                "template-demo-section-publications",
                SectionKind::Publication,
                "Publications & Talks",
                &["template-demo-publication-operational-readiness"],
            ),
            section(
                "template-demo-section-memberships",
                SectionKind::Membership,
                "Professional Memberships",
                &["template-demo-membership-acm"],
            ),
            section(
                "template-demo-section-licenses",
                SectionKind::License,
                "Licenses",
                &["template-demo-license-cloud"],
            ),
            section(
                "template-demo-section-references",
                SectionKind::Reference,
                "References",
                &["template-demo-reference-jordan"],
            ),
            section(
                "template-demo-section-interests",
                SectionKind::Interest,
                "Interests",
                &[
                    "template-demo-interest-open-source",
                    "template-demo-interest-trails",
                ],
            ),
        ],
        template_id: "modern".into(),
        template_version: "1.1.0".into(),
        style_overrides: StyleOverrides::default(),
        updated_at: UPDATED_AT.into(),
    }
}

fn section(id: &str, kind: SectionKind, title: &str, source_ids: &[&str]) -> DraftSection {
    DraftSection {
        id: id.into(),
        kind,
        title: title.into(),
        order: match kind {
            SectionKind::Experience => 0,
            SectionKind::Summary => 0,
            SectionKind::Project => 1,
            SectionKind::Education => 2,
            SectionKind::Skill => 3,
            SectionKind::Certification => 4,
            SectionKind::Language => 5,
            SectionKind::Award => 6,
            SectionKind::Volunteering => 7,
            SectionKind::Publication => 8,
            SectionKind::Membership => 9,
            SectionKind::License => 10,
            SectionKind::Reference => 11,
            SectionKind::Interest => 12,
        },
        entries: source_ids
            .iter()
            .enumerate()
            .map(|(order, source_id)| DraftEntry {
                id: format!("{id}-entry-{order}"),
                source_id: (*source_id).into(),
                included: kind != SectionKind::Reference,
                order: order as u32,
                summary_override: None,
            })
            .collect(),
    }
}
