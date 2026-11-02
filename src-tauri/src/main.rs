use resume_domain::{
    apply_template_to_resume, compile, CareerProfile, ResumeDocument, ResumeDraft, VisualDocument,
};
use resume_storage_sqlite::{ImportOutcome, ImportStrategy, SqliteResumeRepository, Workspace};
use serde::Serialize;
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};
use tauri::Manager;
use tempfile::NamedTempFile;

const MAX_BACKUP_BYTES: u64 = 50 * 1024 * 1024;

#[tauri::command]
fn list_template_palettes(template_id: String) -> Vec<resume_domain::Palette> {
    resume_domain::template_palettes(&template_id)
}

fn database_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let directory = app
        .path()
        .app_local_data_dir()
        .map_err(|error| format!("resolve local data directory: {error}"))?;
    fs::create_dir_all(&directory)
        .map_err(|error| format!("create local data directory: {error}"))?;
    Ok(directory.join("resume-studio.sqlite3"))
}

fn repository(app: &tauri::AppHandle) -> Result<SqliteResumeRepository, String> {
    let path = database_path(app)?;
    SqliteResumeRepository::open(path).map_err(|error| error.to_string())
}

#[derive(Serialize)]
struct CompiledWorkspace {
    profile: CareerProfile,
    draft: ResumeDraft,
    document: ResumeDocument,
    visual_document: VisualDocument,
}

#[derive(Serialize)]
struct PreviewResult {
    document: ResumeDocument,
    visual_document: VisualDocument,
}

#[tauri::command]
fn list_drafts(app: tauri::AppHandle, profile_id: String) -> Result<Vec<ResumeDraft>, String> {
    repository(&app)?
        .list_drafts(&profile_id)
        .map_err(|error| error.to_string())
}

#[tauri::command]
fn list_all_drafts(app: tauri::AppHandle) -> Result<Vec<ResumeDraft>, String> {
    repository(&app)?
        .list_all_drafts()
        .map_err(|error| error.to_string())
}

#[tauri::command]
fn list_profiles(app: tauri::AppHandle) -> Result<Vec<CareerProfile>, String> {
    repository(&app)?
        .list_profiles()
        .map_err(|error| error.to_string())
}

#[tauri::command]
fn preview_workspace(profile: CareerProfile, draft: ResumeDraft) -> Result<PreviewResult, String> {
    let document = compile(&profile, &draft).map_err(|error| error.to_string())?;
    let visual_document =
        apply_template_to_resume(&document, &draft.template_id, &draft.style_overrides)
            .map_err(|error| error.to_string())?;
    Ok(PreviewResult {
        document,
        visual_document,
    })
}

#[tauri::command]
fn save_workspace(
    app: tauri::AppHandle,
    profile: CareerProfile,
    draft: ResumeDraft,
) -> Result<ResumeDocument, String> {
    let document = compile(&profile, &draft).map_err(|error| error.to_string())?;
    repository(&app)?
        .save_workspace(&profile, &draft)
        .map_err(|error| error.to_string())?;
    Ok(document)
}

#[tauri::command]
fn save_profile(app: tauri::AppHandle, profile: CareerProfile) -> Result<(), String> {
    repository(&app)?
        .save_profile(&profile)
        .map_err(|error| error.to_string())
}

#[tauri::command]
fn save_draft(app: tauri::AppHandle, draft: ResumeDraft) -> Result<(), String> {
    repository(&app)?
        .save_draft(&draft)
        .map_err(|error| error.to_string())
}

#[tauri::command]
fn load_workspace(
    app: tauri::AppHandle,
    draft_id: String,
) -> Result<Option<CompiledWorkspace>, String> {
    let workspace: Option<Workspace> = repository(&app)?
        .load_workspace(&draft_id)
        .map_err(|error| error.to_string())?;
    workspace
        .map(|workspace| {
            let document =
                compile(&workspace.profile, &workspace.draft).map_err(|error| error.to_string())?;
            let visual_document = apply_template_to_resume(
                &document,
                &workspace.draft.template_id,
                &workspace.draft.style_overrides,
            )
            .map_err(|error| error.to_string())?;
            Ok(CompiledWorkspace {
                profile: workspace.profile,
                draft: workspace.draft,
                document,
                visual_document,
            })
        })
        .transpose()
}

#[tauri::command]
fn export_backup(app: tauri::AppHandle, path: String) -> Result<(), String> {
    let json = repository(&app)?
        .export_backup_json()
        .map_err(|error| format!("prepare backup: {error}"))?;
    atomic_write(Path::new(&path), json.as_bytes())
}

#[tauri::command]
fn import_backup_json(
    app: tauri::AppHandle,
    json: String,
    strategy: ImportStrategy,
) -> Result<ImportOutcome, String> {
    if json.len() as u64 > MAX_BACKUP_BYTES {
        return Err(format!(
            "backup is larger than the {} MiB limit",
            MAX_BACKUP_BYTES / (1024 * 1024)
        ));
    }
    repository(&app)?
        .import_backup_json(&json, strategy)
        .map_err(|error| format!("import backup: {error}"))
}

#[tauri::command]
fn import_backup(
    app: tauri::AppHandle,
    path: String,
    strategy: ImportStrategy,
) -> Result<ImportOutcome, String> {
    if strategy == ImportStrategy::Cancel {
        return Ok(ImportOutcome::Cancelled);
    }
    let file_path = PathBuf::from(path);
    let metadata =
        fs::metadata(&file_path).map_err(|error| format!("read backup metadata: {error}"))?;
    if metadata.len() > MAX_BACKUP_BYTES {
        return Err(format!(
            "backup is larger than the {} MiB limit",
            MAX_BACKUP_BYTES / (1024 * 1024)
        ));
    }
    let json =
        fs::read_to_string(file_path).map_err(|error| format!("read backup file: {error}"))?;
    repository(&app)?
        .import_backup_json(&json, strategy)
        .map_err(|error| format!("import backup: {error}"))
}

fn atomic_write(path: &Path, contents: &[u8]) -> Result<(), String> {
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    if !parent.is_dir() {
        return Err(format!(
            "backup directory does not exist: {}",
            parent.display()
        ));
    }
    let mut temporary = NamedTempFile::new_in(parent)
        .map_err(|error| format!("create temporary backup file: {error}"))?;
    temporary
        .write_all(contents)
        .map_err(|error| format!("write temporary backup file: {error}"))?;
    temporary
        .as_file()
        .sync_all()
        .map_err(|error| format!("sync temporary backup file: {error}"))?;
    temporary
        .persist(path)
        .map_err(|error| format!("replace backup file: {}", error.error))?;
    fs::File::open(parent)
        .and_then(|directory| directory.sync_all())
        .map_err(|error| format!("sync backup directory: {error}"))?;
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            save_workspace,
            list_template_palettes,
            save_profile,
            save_draft,
            load_workspace,
            list_drafts,
            list_all_drafts,
            list_profiles,
            preview_workspace,
            export_backup,
            import_backup_json,
            import_backup
        ])
        .run(tauri::generate_context!())
        .expect("failed to run Tauri application");
}

#[cfg(test)]
mod tests {
    use super::atomic_write;

    #[test]
    fn backup_file_replacement_is_atomic_and_keeps_new_contents() {
        let directory = tempfile::tempdir().expect("temporary directory is available");
        let path = directory.path().join("backup.json");
        std::fs::write(&path, "old backup").expect("initial backup is written");

        atomic_write(&path, b"new backup").expect("backup is replaced");

        assert_eq!(std::fs::read_to_string(path).unwrap(), "new backup");
    }

    #[test]
    fn backup_export_does_not_create_missing_directories() {
        let directory = tempfile::tempdir().expect("temporary directory is available");
        let path = directory.path().join("missing").join("backup.json");

        assert!(atomic_write(&path, b"backup").is_err());
        assert!(!path.parent().unwrap().exists());
    }
}
