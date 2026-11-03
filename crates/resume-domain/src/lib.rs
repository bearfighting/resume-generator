use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use thiserror::Error;

mod template;
pub use template::{
    apply_template, apply_template_to_resume, builtin_template, template_palettes, BuiltinTemplate,
    ContentDocument, ContentEntry, ContentSection, Density, Diagnostic, DiagnosticSeverity,
    FontFamily, HeadingDecoration, Layout, PageSize, Palette, StyleOverrides, TemplateDefinition,
    TemplateError, TemplateStyle, VisualBlock, VisualDocument, VisualEntry,
    TEMPLATE_SCHEMA_VERSION, VISUAL_DOCUMENT_SCHEMA_VERSION,
};

pub const PROFILE_SCHEMA_VERSION: u32 = 3;
pub const DRAFT_SCHEMA_VERSION: u32 = 3;
pub const DOCUMENT_SCHEMA_VERSION: u32 = 3;
pub const BACKUP_SCHEMA_VERSION: u32 = 1;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CareerProfile {
    pub id: String,
    pub schema_version: u32,
    pub personal: PersonalInfo,
    pub experiences: Vec<Experience>,
    #[serde(default)]
    pub education: Vec<Education>,
    #[serde(default)]
    pub projects: Vec<Project>,
    #[serde(default)]
    pub skills: Vec<Skill>,
    #[serde(default)]
    pub certifications: Vec<Certification>,
    #[serde(default)]
    pub languages: Vec<Language>,
    #[serde(default)]
    pub awards: Vec<Award>,
    #[serde(default)]
    pub volunteering: Vec<VolunteerExperience>,
    #[serde(default)]
    pub publications: Vec<Publication>,
    #[serde(default)]
    pub memberships: Vec<Membership>,
    #[serde(default)]
    pub licenses: Vec<License>,
    #[serde(default)]
    pub references: Vec<ProfessionalReference>,
    #[serde(default)]
    pub interests: Vec<Interest>,
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersonalInfo {
    pub full_name: String,
    pub headline: Option<String>,
    pub email: Option<String>,
    #[serde(default)]
    pub phone: Option<String>,
    #[serde(default)]
    pub location: Option<String>,
    #[serde(default)]
    pub website: Option<String>,
    #[serde(default)]
    pub linkedin: Option<String>,
    #[serde(default)]
    pub github: Option<String>,
    #[serde(default)]
    pub professional_summary: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Experience {
    pub id: String,
    pub role: String,
    pub organization: String,
    pub period: String,
    pub summary: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Education {
    pub id: String,
    pub institution: String,
    pub degree: Option<String>,
    pub field_of_study: Option<String>,
    pub period: Option<String>,
    pub summary: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub role: Option<String>,
    pub period: Option<String>,
    pub url: Option<String>,
    pub summary: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Skill {
    pub id: String,
    pub name: String,
    pub level: Option<String>,
    pub summary: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Certification {
    pub id: String,
    pub name: String,
    pub issuer: String,
    pub period: Option<String>,
    pub credential_url: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Language {
    pub id: String,
    pub name: String,
    pub proficiency: Option<String>,
    pub certification: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Award {
    pub id: String,
    pub name: String,
    pub issuer: Option<String>,
    pub period: Option<String>,
    pub summary: String,
    pub url: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct VolunteerExperience {
    pub id: String,
    pub role: String,
    pub organization: String,
    pub period: Option<String>,
    pub summary: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Publication {
    pub id: String,
    pub title: String,
    pub venue: Option<String>,
    pub period: Option<String>,
    pub url: Option<String>,
    pub summary: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Membership {
    pub id: String,
    pub organization: String,
    pub role: Option<String>,
    pub period: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct License {
    pub id: String,
    pub name: String,
    pub issuer: Option<String>,
    pub period: Option<String>,
    pub credential_url: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfessionalReference {
    pub id: String,
    pub name: String,
    pub role: Option<String>,
    pub organization: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Interest {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResumeDraft {
    pub id: String,
    pub schema_version: u32,
    pub profile_id: String,
    pub title: String,
    pub target_role: Option<String>,
    #[serde(default)]
    pub summary_included: bool,
    #[serde(default)]
    pub summary_override: Option<String>,
    pub sections: Vec<DraftSection>,
    pub template_id: String,
    pub template_version: String,
    #[serde(default)]
    pub style_overrides: StyleOverrides,
    pub updated_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DraftSection {
    pub id: String,
    pub kind: SectionKind,
    pub title: String,
    pub order: u32,
    pub entries: Vec<DraftEntry>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SectionKind {
    Summary,
    Experience,
    Education,
    Project,
    Skill,
    Certification,
    Language,
    Award,
    Volunteering,
    Publication,
    Membership,
    License,
    Reference,
    Interest,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DraftEntry {
    pub id: String,
    pub source_id: String,
    pub included: bool,
    pub order: u32,
    pub summary_override: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResumeDocument {
    pub schema_version: u32,
    pub title: String,
    pub target_role: Option<String>,
    pub personal: PersonalInfo,
    pub sections: Vec<SemanticSection>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackupEnvelope {
    pub schema_version: u32,
    pub profiles: Vec<CareerProfile>,
    pub drafts: Vec<ResumeDraft>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticSection {
    pub kind: SectionKind,
    pub title: String,
    pub entries: Vec<SemanticEntry>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticEntry {
    pub source_id: String,
    pub heading: String,
    pub metadata: Vec<String>,
    pub body: String,
}

fn validate_unique_ids<'a>(
    entity: &'static str,
    ids: impl Iterator<Item = &'a str>,
) -> Result<(), DomainError> {
    let mut seen = HashSet::new();
    for id in ids {
        if !seen.insert(id) {
            return Err(DomainError::DuplicateId {
                entity,
                id: id.to_owned(),
            });
        }
    }
    Ok(())
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DomainError {
    #[error("{field} must not be empty")]
    EmptyField { field: &'static str },
    #[error("unsupported {entity} schema version: {version}")]
    UnsupportedSchemaVersion { entity: &'static str, version: u32 },
    #[error("duplicate {entity} id: {id}")]
    DuplicateId { entity: &'static str, id: String },
    #[error("draft profile id {draft_profile_id} does not match profile id {profile_id}")]
    ProfileMismatch {
        draft_profile_id: String,
        profile_id: String,
    },
    #[error("draft entry references unknown {kind:?} id: {id}")]
    UnknownSource { kind: SectionKind, id: String },
    #[error("backup draft references unknown profile id: {0}")]
    UnknownProfile(String),
}

pub fn validate_profile(profile: &CareerProfile) -> Result<(), DomainError> {
    require_text(&profile.id, "profile.id")?;
    require_text(&profile.personal.full_name, "profile.personal.full_name")?;
    require_text(&profile.updated_at, "profile.updated_at")?;
    if profile.schema_version != PROFILE_SCHEMA_VERSION {
        return Err(DomainError::UnsupportedSchemaVersion {
            entity: "profile",
            version: profile.schema_version,
        });
    }

    for experience in &profile.experiences {
        require_text(&experience.id, "experience.id")?;
        require_text(&experience.role, "experience.role")?;
        require_text(&experience.organization, "experience.organization")?;
    }
    validate_unique_ids(
        "experience",
        profile.experiences.iter().map(|item| item.id.as_str()),
    )?;
    for education in &profile.education {
        require_text(&education.id, "education.id")?;
        require_text(&education.institution, "education.institution")?;
    }
    validate_unique_ids(
        "education",
        profile.education.iter().map(|item| item.id.as_str()),
    )?;
    for project in &profile.projects {
        require_text(&project.id, "project.id")?;
        require_text(&project.name, "project.name")?;
    }
    validate_unique_ids(
        "project",
        profile.projects.iter().map(|item| item.id.as_str()),
    )?;
    for skill in &profile.skills {
        require_text(&skill.id, "skill.id")?;
        require_text(&skill.name, "skill.name")?;
    }
    validate_unique_ids("skill", profile.skills.iter().map(|item| item.id.as_str()))?;
    for certification in &profile.certifications {
        require_text(&certification.id, "certification.id")?;
        require_text(&certification.name, "certification.name")?;
        require_text(&certification.issuer, "certification.issuer")?;
    }
    validate_unique_ids(
        "certification",
        profile.certifications.iter().map(|item| item.id.as_str()),
    )?;
    for language in &profile.languages {
        require_text(&language.id, "language.id")?;
        require_text(&language.name, "language.name")?;
    }
    validate_unique_ids(
        "language",
        profile.languages.iter().map(|item| item.id.as_str()),
    )?;
    for award in &profile.awards {
        require_text(&award.id, "award.id")?;
        require_text(&award.name, "award.name")?;
    }
    validate_unique_ids("award", profile.awards.iter().map(|item| item.id.as_str()))?;
    for volunteer in &profile.volunteering {
        require_text(&volunteer.id, "volunteering.id")?;
        require_text(&volunteer.role, "volunteering.role")?;
        require_text(&volunteer.organization, "volunteering.organization")?;
    }
    validate_unique_ids(
        "volunteering",
        profile.volunteering.iter().map(|item| item.id.as_str()),
    )?;
    for publication in &profile.publications {
        require_text(&publication.id, "publication.id")?;
        require_text(&publication.title, "publication.title")?;
    }
    validate_unique_ids(
        "publication",
        profile.publications.iter().map(|item| item.id.as_str()),
    )?;
    for membership in &profile.memberships {
        require_text(&membership.id, "membership.id")?;
        require_text(&membership.organization, "membership.organization")?;
    }
    validate_unique_ids(
        "membership",
        profile.memberships.iter().map(|item| item.id.as_str()),
    )?;
    for license in &profile.licenses {
        require_text(&license.id, "license.id")?;
        require_text(&license.name, "license.name")?;
    }
    validate_unique_ids(
        "license",
        profile.licenses.iter().map(|item| item.id.as_str()),
    )?;
    for reference in &profile.references {
        require_text(&reference.id, "reference.id")?;
        require_text(&reference.name, "reference.name")?;
    }
    validate_unique_ids(
        "reference",
        profile.references.iter().map(|item| item.id.as_str()),
    )?;
    for interest in &profile.interests {
        require_text(&interest.id, "interest.id")?;
        require_text(&interest.name, "interest.name")?;
    }
    validate_unique_ids(
        "interest",
        profile.interests.iter().map(|item| item.id.as_str()),
    )?;
    Ok(())
}

pub fn upgrade_profile_schema(mut profile: CareerProfile) -> Result<CareerProfile, DomainError> {
    match profile.schema_version {
        1 | 2 => profile.schema_version = PROFILE_SCHEMA_VERSION,
        PROFILE_SCHEMA_VERSION => {}
        version => {
            return Err(DomainError::UnsupportedSchemaVersion {
                entity: "profile",
                version,
            });
        }
    }
    validate_profile(&profile)?;
    Ok(profile)
}

pub fn upgrade_draft_schema(mut draft: ResumeDraft) -> Result<ResumeDraft, DomainError> {
    match draft.schema_version {
        1 | 2 => draft.schema_version = DRAFT_SCHEMA_VERSION,
        DRAFT_SCHEMA_VERSION => {}
        version => {
            return Err(DomainError::UnsupportedSchemaVersion {
                entity: "draft",
                version,
            });
        }
    }
    Ok(draft)
}

pub fn validate_draft(profile: &CareerProfile, draft: &ResumeDraft) -> Result<(), DomainError> {
    validate_profile(profile)?;
    require_text(&draft.id, "draft.id")?;
    require_text(&draft.title, "draft.title")?;
    require_text(&draft.profile_id, "draft.profile_id")?;
    require_text(&draft.template_id, "draft.template_id")?;
    require_text(&draft.template_version, "draft.template_version")?;
    require_text(&draft.updated_at, "draft.updated_at")?;
    if draft.schema_version != DRAFT_SCHEMA_VERSION {
        return Err(DomainError::UnsupportedSchemaVersion {
            entity: "draft",
            version: draft.schema_version,
        });
    }
    if draft.profile_id != profile.id {
        return Err(DomainError::ProfileMismatch {
            draft_profile_id: draft.profile_id.clone(),
            profile_id: profile.id.clone(),
        });
    }

    let mut section_ids = HashSet::new();
    let mut entry_ids = HashSet::new();
    for section in &draft.sections {
        require_text(&section.id, "draft_section.id")?;
        require_text(&section.title, "draft_section.title")?;
        if !section_ids.insert(section.id.as_str()) {
            return Err(DomainError::DuplicateId {
                entity: "draft section",
                id: section.id.clone(),
            });
        }
        for entry in &section.entries {
            require_text(&entry.id, "draft_entry.id")?;
            require_text(&entry.source_id, "draft_entry.source_id")?;
            if !entry_ids.insert(entry.id.as_str()) {
                return Err(DomainError::DuplicateId {
                    entity: "draft entry",
                    id: entry.id.clone(),
                });
            }
            if !profile_has_source(profile, section.kind, &entry.source_id) {
                return Err(DomainError::UnknownSource {
                    kind: section.kind,
                    id: entry.source_id.clone(),
                });
            }
        }
    }
    Ok(())
}

pub fn compile(
    profile: &CareerProfile,
    draft: &ResumeDraft,
) -> Result<ResumeDocument, DomainError> {
    validate_draft(profile, draft)?;
    let mut sections = draft.sections.iter().collect::<Vec<_>>();
    sections.sort_by_key(|section| (section.order, section.id.as_str()));

    let mut semantic_sections = Vec::new();
    if draft.summary_included {
        let summary = draft
            .summary_override
            .as_deref()
            .filter(|value| !value.trim().is_empty())
            .or(profile.personal.professional_summary.as_deref())
            .filter(|value| !value.trim().is_empty());
        if let Some(summary) = summary {
            semantic_sections.push(SemanticSection {
                kind: SectionKind::Summary,
                title: "Summary".into(),
                entries: vec![SemanticEntry {
                    source_id: "profile-summary".into(),
                    heading: "Summary".into(),
                    metadata: Vec::new(),
                    body: summary.to_owned(),
                }],
            });
        }
    }
    for section in sections {
        let mut entries = section.entries.iter().collect::<Vec<_>>();
        entries.sort_by_key(|entry| (entry.order, entry.id.as_str()));
        let mut semantic_entries = Vec::new();
        for entry in entries.into_iter().filter(|entry| entry.included) {
            semantic_entries.push(compile_entry(profile, section.kind, entry)?);
        }
        if !semantic_entries.is_empty() {
            semantic_sections.push(SemanticSection {
                kind: section.kind,
                title: section.title.clone(),
                entries: semantic_entries,
            });
        }
    }

    Ok(ResumeDocument {
        schema_version: DOCUMENT_SCHEMA_VERSION,
        title: draft.title.clone(),
        target_role: draft.target_role.clone(),
        personal: profile.personal.clone(),
        sections: semantic_sections,
    })
}

fn profile_has_source(profile: &CareerProfile, kind: SectionKind, id: &str) -> bool {
    match kind {
        SectionKind::Experience => profile.experiences.iter().any(|item| item.id == id),
        SectionKind::Education => profile.education.iter().any(|item| item.id == id),
        SectionKind::Project => profile.projects.iter().any(|item| item.id == id),
        SectionKind::Skill => profile.skills.iter().any(|item| item.id == id),
        SectionKind::Certification => profile.certifications.iter().any(|item| item.id == id),
        SectionKind::Summary => false,
        SectionKind::Language => profile.languages.iter().any(|item| item.id == id),
        SectionKind::Award => profile.awards.iter().any(|item| item.id == id),
        SectionKind::Volunteering => profile.volunteering.iter().any(|item| item.id == id),
        SectionKind::Publication => profile.publications.iter().any(|item| item.id == id),
        SectionKind::Membership => profile.memberships.iter().any(|item| item.id == id),
        SectionKind::License => profile.licenses.iter().any(|item| item.id == id),
        SectionKind::Reference => profile.references.iter().any(|item| item.id == id),
        SectionKind::Interest => profile.interests.iter().any(|item| item.id == id),
    }
}

fn compile_entry(
    profile: &CareerProfile,
    kind: SectionKind,
    entry: &DraftEntry,
) -> Result<SemanticEntry, DomainError> {
    let missing = || DomainError::UnknownSource {
        kind,
        id: entry.source_id.clone(),
    };
    let (heading, metadata, source_body) = match kind {
        SectionKind::Experience => {
            let source = profile
                .experiences
                .iter()
                .find(|item| item.id == entry.source_id)
                .ok_or_else(missing)?;
            (
                format!("{} — {}", source.role, source.organization),
                nonempty_options([Some(source.period.as_str())]),
                source.summary.clone(),
            )
        }
        SectionKind::Education => {
            let source = profile
                .education
                .iter()
                .find(|item| item.id == entry.source_id)
                .ok_or_else(missing)?;
            (
                source.institution.clone(),
                nonempty_options([
                    source.degree.as_deref(),
                    source.field_of_study.as_deref(),
                    source.period.as_deref(),
                ]),
                source.summary.clone(),
            )
        }
        SectionKind::Project => {
            let source = profile
                .projects
                .iter()
                .find(|item| item.id == entry.source_id)
                .ok_or_else(missing)?;
            (
                source.name.clone(),
                nonempty_options([
                    source.role.as_deref(),
                    source.period.as_deref(),
                    source.url.as_deref(),
                ]),
                source.summary.clone(),
            )
        }
        SectionKind::Skill => {
            let source = profile
                .skills
                .iter()
                .find(|item| item.id == entry.source_id)
                .ok_or_else(missing)?;
            (
                source.name.clone(),
                nonempty_options([source.level.as_deref()]),
                source.summary.clone(),
            )
        }
        SectionKind::Certification => {
            let source = profile
                .certifications
                .iter()
                .find(|item| item.id == entry.source_id)
                .ok_or_else(missing)?;
            (
                source.name.clone(),
                nonempty_options([
                    Some(source.issuer.as_str()),
                    source.period.as_deref(),
                    source.credential_url.as_deref(),
                ]),
                String::new(),
            )
        }
        SectionKind::Summary => return Err(missing()),
        SectionKind::Language => {
            let source = profile
                .languages
                .iter()
                .find(|item| item.id == entry.source_id)
                .ok_or_else(missing)?;
            (
                source.name.clone(),
                nonempty_options([
                    source.proficiency.as_deref(),
                    source.certification.as_deref(),
                ]),
                String::new(),
            )
        }
        SectionKind::Award => {
            let source = profile
                .awards
                .iter()
                .find(|item| item.id == entry.source_id)
                .ok_or_else(missing)?;
            (
                source.name.clone(),
                nonempty_options([
                    source.issuer.as_deref(),
                    source.period.as_deref(),
                    source.url.as_deref(),
                ]),
                source.summary.clone(),
            )
        }
        SectionKind::Volunteering => {
            let source = profile
                .volunteering
                .iter()
                .find(|item| item.id == entry.source_id)
                .ok_or_else(missing)?;
            (
                format!("{} — {}", source.role, source.organization),
                nonempty_options([source.period.as_deref()]),
                source.summary.clone(),
            )
        }
        SectionKind::Publication => {
            let source = profile
                .publications
                .iter()
                .find(|item| item.id == entry.source_id)
                .ok_or_else(missing)?;
            (
                source.title.clone(),
                nonempty_options([
                    source.venue.as_deref(),
                    source.period.as_deref(),
                    source.url.as_deref(),
                ]),
                source.summary.clone(),
            )
        }
        SectionKind::Membership => {
            let source = profile
                .memberships
                .iter()
                .find(|item| item.id == entry.source_id)
                .ok_or_else(missing)?;
            (
                source.organization.clone(),
                nonempty_options([source.role.as_deref(), source.period.as_deref()]),
                String::new(),
            )
        }
        SectionKind::License => {
            let source = profile
                .licenses
                .iter()
                .find(|item| item.id == entry.source_id)
                .ok_or_else(missing)?;
            (
                source.name.clone(),
                nonempty_options([
                    source.issuer.as_deref(),
                    source.period.as_deref(),
                    source.credential_url.as_deref(),
                ]),
                String::new(),
            )
        }
        SectionKind::Reference => {
            let source = profile
                .references
                .iter()
                .find(|item| item.id == entry.source_id)
                .ok_or_else(missing)?;
            (
                source.name.clone(),
                nonempty_options([
                    source.role.as_deref(),
                    source.organization.as_deref(),
                    source.email.as_deref(),
                    source.phone.as_deref(),
                ]),
                String::new(),
            )
        }
        SectionKind::Interest => {
            let source = profile
                .interests
                .iter()
                .find(|item| item.id == entry.source_id)
                .ok_or_else(missing)?;
            (source.name.clone(), Vec::new(), String::new())
        }
    };
    Ok(SemanticEntry {
        source_id: entry.source_id.clone(),
        heading,
        metadata,
        body: entry.summary_override.clone().unwrap_or(source_body),
    })
}

fn nonempty_options<const N: usize>(values: [Option<&str>; N]) -> Vec<String> {
    values
        .into_iter()
        .flatten()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
        .collect()
}

pub fn validate_backup(backup: &BackupEnvelope) -> Result<(), DomainError> {
    if backup.schema_version != BACKUP_SCHEMA_VERSION {
        return Err(DomainError::UnsupportedSchemaVersion {
            entity: "backup",
            version: backup.schema_version,
        });
    }

    let mut profiles = std::collections::HashMap::new();
    for profile in &backup.profiles {
        validate_profile(profile)?;
        if profiles.insert(profile.id.as_str(), profile).is_some() {
            return Err(DomainError::DuplicateId {
                entity: "profile",
                id: profile.id.clone(),
            });
        }
    }

    let mut draft_ids = HashSet::new();
    for draft in &backup.drafts {
        if !draft_ids.insert(draft.id.as_str()) {
            return Err(DomainError::DuplicateId {
                entity: "draft",
                id: draft.id.clone(),
            });
        }
        let profile = profiles
            .get(draft.profile_id.as_str())
            .ok_or_else(|| DomainError::UnknownProfile(draft.profile_id.clone()))?;
        validate_draft(profile, draft)?;
    }
    Ok(())
}

fn require_text(value: &str, field: &'static str) -> Result<(), DomainError> {
    if value.trim().is_empty() {
        return Err(DomainError::EmptyField { field });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn profile() -> CareerProfile {
        CareerProfile {
            id: "profile-1".into(),
            schema_version: PROFILE_SCHEMA_VERSION,
            personal: PersonalInfo {
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
            experiences: vec![
                Experience {
                    id: "exp-1".into(),
                    role: "Engineer".into(),
                    organization: "Northstar".into(),
                    period: "2020–2024".into(),
                    summary: "Built local software".into(),
                },
                Experience {
                    id: "exp-2".into(),
                    role: "Developer".into(),
                    organization: "Paperplane".into(),
                    period: "2017–2020".into(),
                    summary: "Improved accessibility".into(),
                },
            ],
            education: Vec::new(),
            projects: Vec::new(),
            skills: Vec::new(),
            certifications: Vec::new(),
            languages: Vec::new(),
            awards: Vec::new(),
            volunteering: Vec::new(),
            publications: Vec::new(),
            memberships: Vec::new(),
            licenses: Vec::new(),
            references: Vec::new(),
            interests: Vec::new(),
            updated_at: "2026-11-02T00:00:00Z".into(),
        }
    }

    fn draft() -> ResumeDraft {
        ResumeDraft {
            id: "draft-1".into(),
            schema_version: DRAFT_SCHEMA_VERSION,
            profile_id: "profile-1".into(),
            title: "Product Engineer Resume".into(),
            target_role: Some("Senior Product Engineer".into()),
            summary_included: false,
            summary_override: None,
            sections: vec![DraftSection {
                id: "section-experience".into(),
                kind: SectionKind::Experience,
                title: "Experience".into(),
                order: 0,
                entries: vec![
                    DraftEntry {
                        id: "entry-2".into(),
                        source_id: "exp-2".into(),
                        included: true,
                        order: 1,
                        summary_override: None,
                    },
                    DraftEntry {
                        id: "entry-1".into(),
                        source_id: "exp-1".into(),
                        included: true,
                        order: 0,
                        summary_override: Some("Built private-by-default desktop software".into()),
                    },
                ],
            }],
            template_id: "classic".into(),
            template_version: "1.0.0".into(),
            style_overrides: StyleOverrides::default(),
            updated_at: "2026-11-02T00:00:00Z".into(),
        }
    }

    #[test]
    fn compile_applies_order_and_draft_override_without_changing_profile() {
        let profile = profile();
        let draft = draft();
        let document = compile(&profile, &draft).expect("valid fixture compiles");

        let entries = &document.sections[0].entries;
        assert_eq!(entries[0].source_id, "exp-1");
        assert_eq!(entries[0].body, "Built private-by-default desktop software");
        assert_eq!(entries[1].source_id, "exp-2");
        assert_eq!(profile.experiences[0].summary, "Built local software");
    }

    #[test]
    fn compile_omits_hidden_entries() {
        let profile = profile();
        let mut draft = draft();
        draft.sections[0].entries[0].included = false;

        let document = compile(&profile, &draft).expect("valid fixture compiles");
        assert_eq!(document.sections[0].entries.len(), 1);
        assert_eq!(document.sections[0].entries[0].source_id, "exp-1");
    }

    #[test]
    fn compile_supports_an_empty_profile_and_draft() {
        let mut profile = profile();
        profile.experiences.clear();
        let mut draft = draft();
        draft.sections.clear();

        let document = compile(&profile, &draft).expect("empty resume compiles");
        assert_eq!(document.title, draft.title);
        assert!(document.sections.is_empty());
    }

    #[test]
    fn compile_omits_sections_without_included_entries() {
        let profile = profile();
        let mut draft = draft();
        for entry in &mut draft.sections[0].entries {
            entry.included = false;
        }

        let document = compile(&profile, &draft).expect("valid fixture compiles");
        assert!(document.sections.is_empty());
    }

    #[test]
    fn compile_orders_sections_and_entries_by_order_then_id() {
        let profile = profile();
        let mut draft = draft();
        draft.sections.push(DraftSection {
            id: "section-a-summary".into(),
            kind: SectionKind::Experience,
            title: "Earlier section".into(),
            order: 0,
            entries: vec![DraftEntry {
                id: "entry-a".into(),
                source_id: "exp-2".into(),
                included: true,
                order: 0,
                summary_override: None,
            }],
        });
        draft.sections[0].order = 0;
        draft.sections[0].entries[0].order = 0;

        let document = compile(&profile, &draft).expect("valid fixture compiles");
        assert_eq!(document.sections[0].title, "Earlier section");
        assert_eq!(document.sections[1].title, "Experience");
        assert_eq!(document.sections[1].entries[0].source_id, "exp-1");
        assert_eq!(document.sections[1].entries[1].source_id, "exp-2");
    }

    #[test]
    fn compile_preserves_long_unicode_override_without_mutating_inputs() {
        let profile = profile();
        let mut draft = draft();
        let override_text = "履歴書 résumé 🚀 工程 ".repeat(400);
        draft.sections[0].entries[0].summary_override = Some(override_text.clone());
        let original_profile = profile.clone();
        let original_draft = draft.clone();

        let document = compile(&profile, &draft).expect("long Unicode text compiles");
        assert_eq!(document.sections[0].entries[1].body, override_text);
        assert_eq!(profile, original_profile);
        assert_eq!(draft, original_draft);
    }

    #[test]
    fn multiple_drafts_compile_independently_from_the_same_profile() {
        let profile = profile();
        let mut first_draft = draft();
        let mut second_draft = draft();
        first_draft.sections[0].entries[1].summary_override = Some("First tailored copy".into());
        second_draft.sections[0].entries[1].summary_override = Some("Second tailored copy".into());
        second_draft.title = "Second resume".into();

        let first_document =
            compile(&profile, &first_draft).expect("first draft compiles independently");
        let second_document =
            compile(&profile, &second_draft).expect("second draft compiles independently");

        assert_eq!(first_document.title, "Product Engineer Resume");
        assert_eq!(second_document.title, "Second resume");
        assert_eq!(
            first_document.sections[0].entries[0].body,
            "First tailored copy"
        );
        assert_eq!(
            second_document.sections[0].entries[0].body,
            "Second tailored copy"
        );
        assert_eq!(profile.experiences[0].summary, "Built local software");
    }

    #[test]
    fn compile_rejects_a_dangling_reference() {
        let profile = profile();
        let mut draft = draft();
        draft.sections[0].entries[0].source_id = "missing".into();

        assert_eq!(
            compile(&profile, &draft),
            Err(DomainError::UnknownSource {
                kind: SectionKind::Experience,
                id: "missing".into(),
            })
        );
    }

    #[test]
    fn validation_rejects_duplicate_source_ids() {
        let mut profile = profile();
        profile.experiences[1].id = profile.experiences[0].id.clone();

        assert_eq!(
            validate_profile(&profile),
            Err(DomainError::DuplicateId {
                entity: "experience",
                id: "exp-1".into(),
            })
        );
    }

    #[test]
    fn validation_rejects_a_draft_for_another_profile() {
        let profile = profile();
        let mut draft = draft();
        draft.profile_id = "profile-2".into();

        assert_eq!(
            validate_draft(&profile, &draft),
            Err(DomainError::ProfileMismatch {
                draft_profile_id: "profile-2".into(),
                profile_id: "profile-1".into(),
            })
        );
    }

    #[test]
    fn backup_validation_rejects_a_draft_with_an_unknown_profile() {
        let mut draft = draft();
        draft.profile_id = "missing-profile".into();
        let backup = BackupEnvelope {
            schema_version: BACKUP_SCHEMA_VERSION,
            profiles: vec![profile()],
            drafts: vec![draft],
        };

        assert_eq!(
            validate_backup(&backup),
            Err(DomainError::UnknownProfile("missing-profile".into()))
        );
    }

    #[test]
    fn profile_and_draft_round_trip_through_json() {
        let profile = profile();
        let draft = draft();
        let profile_json = serde_json::to_string(&profile).expect("profile serializes");
        let draft_json = serde_json::to_string(&draft).expect("draft serializes");

        assert_eq!(
            serde_json::from_str::<CareerProfile>(&profile_json).expect("profile deserializes"),
            profile
        );
        assert_eq!(
            serde_json::from_str::<ResumeDraft>(&draft_json).expect("draft deserializes"),
            draft
        );
    }

    #[test]
    fn profile_v1_json_upgrades_with_empty_new_collections() {
        let mut legacy = serde_json::to_value(profile()).expect("fixture serializes");
        legacy["schema_version"] = serde_json::json!(1);
        legacy.as_object_mut().unwrap().remove("education");
        legacy.as_object_mut().unwrap().remove("projects");
        legacy.as_object_mut().unwrap().remove("skills");
        legacy.as_object_mut().unwrap().remove("certifications");

        let decoded: CareerProfile =
            serde_json::from_value(legacy).expect("legacy profile deserializes");
        let upgraded = upgrade_profile_schema(decoded).expect("v1 upgrades to v2");
        assert_eq!(upgraded.schema_version, PROFILE_SCHEMA_VERSION);
        assert!(upgraded.education.is_empty());
        assert!(upgraded.projects.is_empty());
        assert!(upgraded.skills.is_empty());
        assert!(upgraded.certifications.is_empty());
    }

    #[test]
    fn draft_v1_json_upgrades_with_default_style_overrides() {
        let mut legacy = serde_json::to_value(draft()).expect("draft serializes");
        legacy["schema_version"] = serde_json::json!(1);
        legacy.as_object_mut().unwrap().remove("style_overrides");
        let decoded: ResumeDraft =
            serde_json::from_value(legacy).expect("legacy draft deserializes");
        let upgraded = upgrade_draft_schema(decoded).expect("v1 draft upgrades to v2");
        assert_eq!(upgraded.schema_version, DRAFT_SCHEMA_VERSION);
        assert_eq!(upgraded.style_overrides, StyleOverrides::default());
    }

    #[test]
    fn compiler_supports_every_profile_category_with_typed_references() {
        let mut profile = profile();
        profile.education.push(Education {
            id: "shared-id".into(),
            institution: "Example University".into(),
            degree: Some("BSc".into()),
            field_of_study: Some("Computer Science".into()),
            period: Some("2012–2016".into()),
            summary: "Graduated with honors".into(),
        });
        profile.projects.push(Project {
            id: "shared-id".into(),
            name: "Offline Workspace".into(),
            role: Some("Creator".into()),
            period: Some("2024–2026".into()),
            url: Some("https://example.test/project".into()),
            summary: "Built an offline-first product".into(),
        });
        profile.skills.push(Skill {
            id: "shared-id".into(),
            name: "Rust".into(),
            level: Some("Advanced".into()),
            summary: "Systems programming".into(),
        });
        profile.certifications.push(Certification {
            id: "shared-id".into(),
            name: "Cloud Practitioner".into(),
            issuer: "Example Cloud".into(),
            period: Some("2025".into()),
            credential_url: None,
        });
        let mut draft = draft();
        for (kind, title) in [
            (SectionKind::Education, "Education"),
            (SectionKind::Project, "Projects"),
            (SectionKind::Skill, "Skills"),
            (SectionKind::Certification, "Certifications"),
        ] {
            draft.sections.push(DraftSection {
                id: format!("section-{title}"),
                kind,
                title: title.into(),
                order: draft.sections.len() as u32,
                entries: vec![DraftEntry {
                    id: format!("entry-{title}"),
                    source_id: "shared-id".into(),
                    included: true,
                    order: 0,
                    summary_override: None,
                }],
            });
        }

        let document = compile(&profile, &draft).expect("mixed category draft compiles");
        assert_eq!(document.schema_version, DOCUMENT_SCHEMA_VERSION);
        assert_eq!(document.sections.len(), 5);
        assert_eq!(
            document.sections[1].entries[0].heading,
            "Example University"
        );
        assert_eq!(document.sections[2].entries[0].heading, "Offline Workspace");
        assert_eq!(document.sections[3].entries[0].heading, "Rust");
        assert_eq!(document.sections[4].entries[0].metadata[0], "Example Cloud");
    }
}
