use resume_domain::{
    compile, upgrade_draft_schema, upgrade_profile_schema, validate_backup, validate_profile,
    BackupEnvelope, CareerProfile, DomainError, ResumeDraft, BACKUP_SCHEMA_VERSION,
};
use rusqlite::{params, Connection, OptionalExtension, Transaction};
use serde::{Deserialize, Serialize};
use std::path::Path;
use thiserror::Error;

pub const DATABASE_SCHEMA_VERSION: i64 = 1;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("SQLite operation failed: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("JSON serialization failed: {0}")]
    Json(#[from] serde_json::Error),
    #[error("domain validation failed: {0}")]
    Domain(#[from] DomainError),
    #[error("database schema version {0} is newer than this application supports")]
    UnsupportedDatabaseVersion(i64),
    #[error(
        "resume draft {draft_id} references profile {profile_id}, but that profile was not found"
    )]
    MissingProfileForDraft {
        draft_id: String,
        profile_id: String,
    },
    #[error("database record id does not match its stored payload")]
    RecordIdMismatch,
    #[error("updating profile would invalidate draft {draft_id}: {reason}")]
    DraftInvalidated { draft_id: String, reason: String },
    #[error("filesystem operation failed: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Workspace {
    pub profile: CareerProfile,
    pub draft: ResumeDraft,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportStrategy {
    Replace,
    Cancel,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportOutcome {
    Imported {
        profile_count: usize,
        draft_count: usize,
    },
    Cancelled,
}

pub struct SqliteResumeRepository {
    connection: Connection,
}

impl SqliteResumeRepository {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, StorageError> {
        let path = path.as_ref();
        if let Some(parent) = path
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
        {
            std::fs::create_dir_all(parent)?;
        }

        let connection = Connection::open(path)?;
        connection.pragma_update(None, "foreign_keys", "ON")?;
        connection.busy_timeout(std::time::Duration::from_secs(5))?;
        migrate(&connection)?;

        Ok(Self { connection })
    }

    pub fn get_profile(&self, profile_id: &str) -> Result<Option<CareerProfile>, StorageError> {
        let record: Option<(String, String)> = self
            .connection
            .query_row(
                "SELECT id, payload FROM career_profiles WHERE id = ?1",
                params![profile_id],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .optional()?;
        record
            .map(|(id, payload)| parse_profile_record(&id, &payload))
            .transpose()
    }

    pub fn list_profiles(&self) -> Result<Vec<CareerProfile>, StorageError> {
        let mut statement = self
            .connection
            .prepare("SELECT id, payload FROM career_profiles ORDER BY id")?;
        let records = statement
            .query_map([], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })?
            .collect::<Result<Vec<_>, _>>()?;
        records
            .iter()
            .map(|(id, payload)| parse_profile_record(id, payload))
            .collect()
    }

    pub fn save_profile(&mut self, profile: &CareerProfile) -> Result<(), StorageError> {
        validate_profile(profile)?;
        self.ensure_existing_drafts_valid(profile, None)?;
        let payload = serde_json::to_string(profile)?;
        let transaction = self.connection.transaction()?;
        upsert_profile(&transaction, profile, &payload)?;
        transaction.commit()?;
        Ok(())
    }

    pub fn delete_profile(&mut self, profile_id: &str) -> Result<bool, StorageError> {
        let transaction = self
            .connection
            .transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)?;
        transaction.execute(
            "DELETE FROM resume_drafts WHERE profile_id = ?1",
            params![profile_id],
        )?;
        let deleted = transaction.execute(
            "DELETE FROM career_profiles WHERE id = ?1",
            params![profile_id],
        )?;
        transaction.commit()?;
        Ok(deleted > 0)
    }

    pub fn get_draft(&self, draft_id: &str) -> Result<Option<ResumeDraft>, StorageError> {
        let record: Option<(String, String)> = self
            .connection
            .query_row(
                "SELECT id, payload FROM resume_drafts WHERE id = ?1",
                params![draft_id],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .optional()?;
        record
            .map(|(id, payload)| parse_draft_record(&id, &payload))
            .transpose()
    }

    pub fn list_drafts(&self, profile_id: &str) -> Result<Vec<ResumeDraft>, StorageError> {
        let mut statement = self
            .connection
            .prepare("SELECT id, payload FROM resume_drafts WHERE profile_id = ?1 ORDER BY id")?;
        let records = statement
            .query_map(params![profile_id], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })?
            .collect::<Result<Vec<_>, _>>()?;
        records
            .iter()
            .map(|(id, payload)| parse_draft_record(id, payload))
            .collect()
    }

    pub fn list_all_drafts(&self) -> Result<Vec<ResumeDraft>, StorageError> {
        let mut statement = self
            .connection
            .prepare("SELECT id, payload FROM resume_drafts ORDER BY id")?;
        let records = statement
            .query_map([], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
            })?
            .collect::<Result<Vec<_>, _>>()?;
        records
            .iter()
            .map(|(id, payload)| parse_draft_record(id, payload))
            .collect()
    }

    pub fn save_draft(&mut self, draft: &ResumeDraft) -> Result<(), StorageError> {
        let Some(profile) = self.get_profile(&draft.profile_id)? else {
            return Err(StorageError::MissingProfileForDraft {
                draft_id: draft.id.clone(),
                profile_id: draft.profile_id.clone(),
            });
        };
        compile(&profile, draft)?;
        let payload = serde_json::to_string(draft)?;
        let transaction = self.connection.transaction()?;
        upsert_draft(&transaction, draft, &payload)?;
        transaction.commit()?;
        Ok(())
    }

    pub fn delete_draft(&mut self, draft_id: &str) -> Result<bool, StorageError> {
        let deleted = self
            .connection
            .execute("DELETE FROM resume_drafts WHERE id = ?1", params![draft_id])?;
        Ok(deleted > 0)
    }

    pub fn export_backup_json(&self) -> Result<String, StorageError> {
        let profiles = self.list_profiles()?;
        let mut drafts = Vec::new();
        for profile in &profiles {
            drafts.extend(self.list_drafts(&profile.id)?);
        }
        let backup = BackupEnvelope {
            schema_version: BACKUP_SCHEMA_VERSION,
            profiles,
            drafts,
        };
        validate_backup(&backup)?;
        Ok(serde_json::to_string_pretty(&backup)?)
    }

    pub fn import_backup_json(
        &mut self,
        json: &str,
        strategy: ImportStrategy,
    ) -> Result<ImportOutcome, StorageError> {
        if strategy == ImportStrategy::Cancel {
            return Ok(ImportOutcome::Cancelled);
        }
        let mut backup: BackupEnvelope = serde_json::from_str(json)?;
        for profile in &mut backup.profiles {
            *profile = upgrade_profile_schema(profile.clone())?;
        }
        for draft in &mut backup.drafts {
            *draft = upgrade_draft_schema(draft.clone())?;
        }
        validate_backup(&backup)?;
        let profile_records = backup
            .profiles
            .iter()
            .map(|profile| Ok((profile, serde_json::to_string(profile)?)))
            .collect::<Result<Vec<_>, serde_json::Error>>()?;
        let draft_records = backup
            .drafts
            .iter()
            .map(|draft| Ok((draft, serde_json::to_string(draft)?)))
            .collect::<Result<Vec<_>, serde_json::Error>>()?;
        let profile_count = profile_records.len();
        let draft_count = draft_records.len();

        let transaction = self.connection.transaction()?;
        transaction.execute("DELETE FROM resume_drafts", [])?;
        transaction.execute("DELETE FROM career_profiles", [])?;
        for (profile, payload) in profile_records {
            upsert_profile(&transaction, profile, &payload)?;
        }
        for (draft, payload) in draft_records {
            upsert_draft(&transaction, draft, &payload)?;
        }
        transaction.commit()?;
        Ok(ImportOutcome::Imported {
            profile_count,
            draft_count,
        })
    }

    pub fn save_workspace(
        &mut self,
        profile: &CareerProfile,
        draft: &ResumeDraft,
    ) -> Result<(), StorageError> {
        compile(profile, draft)?;
        self.ensure_existing_drafts_valid(profile, Some(&draft.id))?;
        let profile_payload = serde_json::to_string(profile)?;
        let draft_payload = serde_json::to_string(draft)?;
        let transaction = self.connection.transaction()?;
        upsert_profile(&transaction, profile, &profile_payload)?;
        upsert_draft(&transaction, draft, &draft_payload)?;
        transaction.commit()?;
        Ok(())
    }

    fn ensure_existing_drafts_valid(
        &self,
        profile: &CareerProfile,
        except_draft_id: Option<&str>,
    ) -> Result<(), StorageError> {
        for existing_draft in self.list_drafts(&profile.id)? {
            if Some(existing_draft.id.as_str()) == except_draft_id {
                continue;
            }
            compile(profile, &existing_draft).map_err(|error| StorageError::DraftInvalidated {
                draft_id: existing_draft.id,
                reason: error.to_string(),
            })?;
        }
        Ok(())
    }

    pub fn load_workspace(&self, draft_id: &str) -> Result<Option<Workspace>, StorageError> {
        let draft_payload: Option<String> = self
            .connection
            .query_row(
                "SELECT payload FROM resume_drafts WHERE id = ?1",
                params![draft_id],
                |row| row.get(0),
            )
            .optional()?;
        let Some(draft_payload) = draft_payload else {
            return Ok(None);
        };
        let draft = upgrade_draft_schema(serde_json::from_str(&draft_payload)?)?;
        if draft.id != draft_id {
            return Err(StorageError::RecordIdMismatch);
        }

        let profile_payload: Option<String> = self
            .connection
            .query_row(
                "SELECT payload FROM career_profiles WHERE id = ?1",
                params![draft.profile_id],
                |row| row.get(0),
            )
            .optional()?;
        let Some(profile_payload) = profile_payload else {
            return Err(StorageError::MissingProfileForDraft {
                draft_id: draft.id,
                profile_id: draft.profile_id,
            });
        };
        let profile = upgrade_profile_schema(serde_json::from_str(&profile_payload)?)?;
        if profile.id != draft.profile_id {
            return Err(StorageError::RecordIdMismatch);
        }

        compile(&profile, &draft)?;
        Ok(Some(Workspace { profile, draft }))
    }
}

fn parse_profile_record(id: &str, payload: &str) -> Result<CareerProfile, StorageError> {
    let profile = upgrade_profile_schema(serde_json::from_str(payload)?)?;
    if profile.id != id {
        return Err(StorageError::RecordIdMismatch);
    }
    validate_profile(&profile)?;
    Ok(profile)
}

fn parse_draft_record(id: &str, payload: &str) -> Result<ResumeDraft, StorageError> {
    let draft = upgrade_draft_schema(serde_json::from_str(payload)?)?;
    if draft.id != id {
        return Err(StorageError::RecordIdMismatch);
    }
    Ok(draft)
}

fn migrate(connection: &Connection) -> Result<(), StorageError> {
    let current_version: i64 =
        connection.pragma_query_value(None, "user_version", |row| row.get(0))?;
    match current_version {
        DATABASE_SCHEMA_VERSION => Ok(()),
        0 => {
            connection.execute_batch(
                "BEGIN IMMEDIATE;
                 CREATE TABLE career_profiles (
                   id TEXT PRIMARY KEY NOT NULL,
                   schema_version INTEGER NOT NULL,
                   updated_at TEXT NOT NULL,
                   payload TEXT NOT NULL
                 );
                 CREATE TABLE resume_drafts (
                   id TEXT PRIMARY KEY NOT NULL,
                   profile_id TEXT NOT NULL REFERENCES career_profiles(id) ON DELETE RESTRICT,
                   schema_version INTEGER NOT NULL,
                   updated_at TEXT NOT NULL,
                   payload TEXT NOT NULL
                 );
                 CREATE INDEX resume_drafts_profile_id ON resume_drafts(profile_id);
                 PRAGMA user_version = 1;
                 COMMIT;",
            )?;
            Ok(())
        }
        version if version > DATABASE_SCHEMA_VERSION => {
            Err(StorageError::UnsupportedDatabaseVersion(version))
        }
        version => Err(StorageError::UnsupportedDatabaseVersion(version)),
    }
}

fn upsert_profile(
    transaction: &Transaction<'_>,
    profile: &CareerProfile,
    payload: &str,
) -> Result<(), rusqlite::Error> {
    transaction.execute(
        "INSERT INTO career_profiles(id, schema_version, updated_at, payload)
         VALUES (?1, ?2, ?3, ?4)
         ON CONFLICT(id) DO UPDATE SET
           schema_version = excluded.schema_version,
           updated_at = excluded.updated_at,
           payload = excluded.payload",
        params![
            profile.id,
            profile.schema_version,
            profile.updated_at,
            payload
        ],
    )?;
    Ok(())
}

fn upsert_draft(
    transaction: &Transaction<'_>,
    draft: &ResumeDraft,
    payload: &str,
) -> Result<(), rusqlite::Error> {
    transaction.execute(
        "INSERT INTO resume_drafts(id, profile_id, schema_version, updated_at, payload)
         VALUES (?1, ?2, ?3, ?4, ?5)
         ON CONFLICT(id) DO UPDATE SET
           profile_id = excluded.profile_id,
           schema_version = excluded.schema_version,
           updated_at = excluded.updated_at,
           payload = excluded.payload",
        params![
            draft.id,
            draft.profile_id,
            draft.schema_version,
            draft.updated_at,
            payload
        ],
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use resume_domain::{
        DraftEntry, DraftSection, Experience, PersonalInfo, SectionKind, DRAFT_SCHEMA_VERSION,
        PROFILE_SCHEMA_VERSION,
    };

    fn sample_workspace() -> Workspace {
        let profile = CareerProfile {
            id: "profile-1".into(),
            schema_version: PROFILE_SCHEMA_VERSION,
            personal: PersonalInfo {
                full_name: "Alex Morgan".into(),
                headline: Some("Product Engineer".into()),
                email: None,
                phone: None,
                location: None,
                website: None,
                linkedin: None,
                github: None,
                professional_summary: None,
            },
            experiences: vec![Experience {
                id: "exp-1".into(),
                role: "Engineer".into(),
                organization: "Northstar".into(),
                period: "2020–2024".into(),
                summary: "Built local software".into(),
            }],
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
        };
        let draft = ResumeDraft {
            id: "draft-1".into(),
            schema_version: DRAFT_SCHEMA_VERSION,
            profile_id: profile.id.clone(),
            title: "Engineer Resume".into(),
            target_role: Some("Senior Engineer".into()),
            summary_included: false,
            summary_override: None,
            sections: vec![DraftSection {
                id: "section-experience".into(),
                kind: SectionKind::Experience,
                title: "Experience".into(),
                order: 0,
                entries: vec![DraftEntry {
                    id: "entry-1".into(),
                    source_id: "exp-1".into(),
                    included: true,
                    order: 0,
                    summary_override: None,
                }],
            }],
            template_id: "classic".into(),
            template_version: "1.0.0".into(),
            style_overrides: resume_domain::StyleOverrides::default(),
            updated_at: "2026-11-02T00:00:00Z".into(),
        };
        Workspace { profile, draft }
    }

    #[test]
    fn workspace_survives_repository_reopen() {
        let directory = tempfile::tempdir().expect("temporary directory is available");
        let path = directory.path().join("resume.sqlite3");
        let expected = sample_workspace();

        let mut repository = SqliteResumeRepository::open(&path).expect("repository opens");
        repository
            .save_workspace(&expected.profile, &expected.draft)
            .expect("workspace saves atomically");
        drop(repository);

        let repository = SqliteResumeRepository::open(&path).expect("repository reopens");
        assert_eq!(
            repository
                .load_workspace("draft-1")
                .expect("workspace loads"),
            Some(expected)
        );
    }

    #[test]
    fn invalid_draft_does_not_overwrite_saved_workspace() {
        let directory = tempfile::tempdir().expect("temporary directory is available");
        let path = directory.path().join("resume.sqlite3");
        let expected = sample_workspace();
        let mut repository = SqliteResumeRepository::open(&path).expect("repository opens");
        repository
            .save_workspace(&expected.profile, &expected.draft)
            .expect("workspace saves");

        let mut invalid_draft = expected.draft.clone();
        invalid_draft.sections[0].entries[0].source_id = "missing".into();
        assert!(repository
            .save_workspace(&expected.profile, &invalid_draft)
            .is_err());
        assert_eq!(
            repository
                .load_workspace("draft-1")
                .expect("original workspace still loads"),
            Some(expected)
        );
    }

    #[test]
    fn newer_database_schema_is_rejected() {
        let directory = tempfile::tempdir().expect("temporary directory is available");
        let path = directory.path().join("resume.sqlite3");
        let connection = Connection::open(&path).expect("database opens");
        connection
            .pragma_update(None, "user_version", DATABASE_SCHEMA_VERSION + 1)
            .expect("future schema version is set");
        drop(connection);

        assert!(matches!(
            SqliteResumeRepository::open(path),
            Err(StorageError::UnsupportedDatabaseVersion(2))
        ));
    }

    #[test]
    fn profile_updates_cannot_break_referenced_drafts() {
        let directory = tempfile::tempdir().expect("temporary directory is available");
        let path = directory.path().join("resume.sqlite3");
        let workspace = sample_workspace();
        let mut repository = SqliteResumeRepository::open(&path).expect("repository opens");
        repository
            .save_workspace(&workspace.profile, &workspace.draft)
            .expect("workspace saves");

        let mut changed_profile = workspace.profile.clone();
        changed_profile.experiences.clear();
        assert!(matches!(
            repository.save_profile(&changed_profile),
            Err(StorageError::DraftInvalidated { .. })
        ));
        assert!(repository.delete_profile("profile-1").unwrap());
        assert_eq!(repository.get_profile("profile-1").unwrap(), None);
        assert!(repository.list_drafts("profile-1").unwrap().is_empty());
    }

    #[test]
    fn deleting_profile_transactionally_deletes_its_drafts() {
        let directory = tempfile::tempdir().expect("temporary directory is available");
        let path = directory.path().join("resume.sqlite3");
        let workspace = sample_workspace();
        let mut repository = SqliteResumeRepository::open(&path).expect("repository opens");
        repository
            .save_workspace(&workspace.profile, &workspace.draft)
            .expect("workspace saves");

        let mut second_draft = workspace.draft.clone();
        second_draft.id = "draft-2".into();
        second_draft.title = "Alternate Resume".into();
        repository
            .save_draft(&second_draft)
            .expect("second draft saves");

        assert_eq!(repository.list_profiles().unwrap(), vec![workspace.profile]);
        assert_eq!(repository.list_drafts("profile-1").unwrap().len(), 2);
        assert!(repository.delete_profile("profile-1").unwrap());
        assert!(repository.list_profiles().unwrap().is_empty());
        assert!(repository.list_drafts("profile-1").unwrap().is_empty());
    }

    #[test]
    fn versioned_backup_round_trips_and_cancel_preserves_data() {
        let directory = tempfile::tempdir().expect("temporary directory is available");
        let source_path = directory.path().join("source.sqlite3");
        let restored_path = directory.path().join("restored.sqlite3");
        let workspace = sample_workspace();

        let mut source = SqliteResumeRepository::open(&source_path).expect("source opens");
        source
            .save_workspace(&workspace.profile, &workspace.draft)
            .expect("workspace saves");
        let mut second_draft = workspace.draft.clone();
        second_draft.id = "draft-2".into();
        second_draft.title = "Alternate Resume".into();
        source
            .save_draft(&second_draft)
            .expect("second draft saves");
        let backup = source.export_backup_json().expect("backup exports");

        let mut restored =
            SqliteResumeRepository::open(&restored_path).expect("restored repository opens");
        assert_eq!(
            restored
                .import_backup_json(&backup, ImportStrategy::Replace)
                .unwrap(),
            ImportOutcome::Imported {
                profile_count: 1,
                draft_count: 2,
            }
        );
        assert_eq!(restored.list_drafts("profile-1").unwrap().len(), 2);

        assert_eq!(
            restored
                .import_backup_json("not valid json", ImportStrategy::Cancel)
                .unwrap(),
            ImportOutcome::Cancelled
        );
        assert_eq!(restored.list_drafts("profile-1").unwrap().len(), 2);
    }

    #[test]
    fn legacy_v1_backup_imports_as_profile_v2() {
        let directory = tempfile::tempdir().expect("temporary directory is available");
        let path = directory.path().join("legacy.sqlite3");
        let workspace = sample_workspace();
        let mut profile = serde_json::to_value(&workspace.profile).expect("profile serializes");
        profile["schema_version"] = serde_json::json!(1);
        let object = profile.as_object_mut().expect("profile is an object");
        object.remove("education");
        object.remove("projects");
        object.remove("skills");
        object.remove("certifications");
        let mut draft = serde_json::to_value(&workspace.draft).expect("draft serializes");
        draft["schema_version"] = serde_json::json!(1);
        draft
            .as_object_mut()
            .expect("draft is an object")
            .remove("style_overrides");
        let legacy = serde_json::json!({
            "schema_version": BACKUP_SCHEMA_VERSION,
            "profiles": [profile],
            "drafts": [draft],
        });
        let mut repository = SqliteResumeRepository::open(&path).expect("repository opens");
        repository
            .import_backup_json(&legacy.to_string(), ImportStrategy::Replace)
            .expect("v1 backup imports");

        let restored = repository
            .get_profile("profile-1")
            .expect("profile reads")
            .expect("profile exists");
        assert_eq!(
            restored.schema_version,
            resume_domain::PROFILE_SCHEMA_VERSION
        );
        assert!(restored.education.is_empty());
        assert!(restored.projects.is_empty());
        assert!(restored.skills.is_empty());
        assert!(restored.certifications.is_empty());
        assert_eq!(
            repository
                .get_draft("draft-1")
                .unwrap()
                .unwrap()
                .schema_version,
            resume_domain::DRAFT_SCHEMA_VERSION
        );
    }

    #[test]
    fn legacy_v1_sqlite_workspace_loads_as_v2() {
        let directory = tempfile::tempdir().expect("temporary directory is available");
        let path = directory.path().join("legacy-records.sqlite3");
        let workspace = sample_workspace();
        let mut profile = serde_json::to_value(&workspace.profile).expect("profile serializes");
        profile["schema_version"] = serde_json::json!(1);
        let profile_object = profile.as_object_mut().expect("profile is an object");
        profile_object.remove("education");
        profile_object.remove("projects");
        profile_object.remove("skills");
        profile_object.remove("certifications");
        let mut draft = serde_json::to_value(&workspace.draft).expect("draft serializes");
        draft["schema_version"] = serde_json::json!(1);
        draft
            .as_object_mut()
            .expect("draft is an object")
            .remove("style_overrides");

        let repository = SqliteResumeRepository::open(&path).expect("repository opens");
        repository.connection.execute(
            "INSERT INTO career_profiles(id, schema_version, updated_at, payload) VALUES (?1, 1, ?2, ?3)",
            params![workspace.profile.id, workspace.profile.updated_at, profile.to_string()],
        ).expect("legacy profile is stored");
        repository.connection.execute(
            "INSERT INTO resume_drafts(id, profile_id, schema_version, updated_at, payload) VALUES (?1, ?2, 1, ?3, ?4)",
            params![workspace.draft.id, workspace.draft.profile_id, workspace.draft.updated_at, draft.to_string()],
        ).expect("legacy draft is stored");

        let loaded = repository
            .load_workspace("draft-1")
            .expect("legacy workspace loads")
            .expect("legacy workspace exists");
        assert_eq!(
            loaded.profile.schema_version,
            resume_domain::PROFILE_SCHEMA_VERSION
        );
        assert_eq!(
            loaded.draft.schema_version,
            resume_domain::DRAFT_SCHEMA_VERSION
        );
        assert_eq!(loaded.profile.experiences, workspace.profile.experiences);
        assert_eq!(
            loaded.draft.style_overrides,
            resume_domain::StyleOverrides::default()
        );
    }

    #[test]
    fn failed_backup_import_rolls_back_all_database_changes() {
        let directory = tempfile::tempdir().expect("temporary directory is available");
        let path = directory.path().join("resume.sqlite3");
        let workspace = sample_workspace();
        let mut repository = SqliteResumeRepository::open(&path).expect("repository opens");
        repository
            .save_workspace(&workspace.profile, &workspace.draft)
            .expect("workspace saves");

        let mut incoming = workspace.clone();
        incoming.profile.personal.full_name = "Replacement Name".into();
        let backup = serde_json::to_string(&BackupEnvelope {
            schema_version: BACKUP_SCHEMA_VERSION,
            profiles: vec![incoming.profile],
            drafts: vec![incoming.draft],
        })
        .expect("backup serializes");
        repository
            .connection
            .execute_batch(
                "CREATE TRIGGER fail_draft_insert BEFORE INSERT ON resume_drafts
                 BEGIN SELECT RAISE(ABORT, 'injected import failure'); END;",
            )
            .expect("failure trigger installs");

        assert!(repository
            .import_backup_json(&backup, ImportStrategy::Replace)
            .is_err());
        assert_eq!(
            repository
                .load_workspace("draft-1")
                .unwrap()
                .unwrap()
                .profile
                .personal
                .full_name,
            "Alex Morgan"
        );
    }
}
