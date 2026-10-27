use std::fs;
use std::path::PathBuf;
use tauri::Manager;
use serde::{Serialize, Deserialize};

fn get_sync_dir(app: &tauri::AppHandle) -> PathBuf {
    let mut dir = app.path().document_dir().expect("failed to resolve documents directory");
    dir.push("BarRecitationSync");
    if !dir.exists() {
        let _ = fs::create_dir_all(&dir);
    }
    dir
}

#[derive(Serialize, Deserialize)]
pub struct DiskSyncMetadata {
    pub exists: bool,
    pub file_name: String,
    pub path: String,
    pub modified_timestamp_ms: u64,
    pub export_date: Option<String>,
    pub deck_count: usize,
    pub mastery_count: usize,
}

#[tauri::command]
fn open_sync_directory(app: tauri::AppHandle) -> Result<(), String> {
    let dir = get_sync_dir(&app);

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(&dir.to_string_lossy().to_string())
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&dir.to_string_lossy().to_string())
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&dir.to_string_lossy().to_string())
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
fn check_subject_sync_status(app: tauri::AppHandle, file_name: String) -> Result<DiskSyncMetadata, String> {
    let mut file_path = get_sync_dir(&app);
    file_path.push(&file_name);

    if !file_path.exists() {
        return Ok(DiskSyncMetadata {
            exists: false,
            file_name,
            path: file_path.to_string_lossy().to_string(),
            modified_timestamp_ms: 0,
            export_date: None,
            deck_count: 0,
            mastery_count: 0,
        });
    }

    let metadata = fs::metadata(&file_path).map_err(|e| e.to_string())?;
    let modified = metadata
        .modified()
        .map_err(|e| e.to_string())?
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_millis() as u64;

    let content = fs::read_to_string(&file_path).map_err(|e| e.to_string())?;
    let json: serde_json::Value = serde_json::from_str(&content).map_err(|e| e.to_string())?;

    let export_date = json.get("exportDate").and_then(|v| v.as_str()).map(|s| s.to_string());
    let deck_count = json.get("decks").and_then(|v| v.as_array()).map(|a| a.len()).unwrap_or(0);
    let mastery_count = json.get("mastery").and_then(|v| v.as_array()).map(|a| a.len()).unwrap_or(0);

    Ok(DiskSyncMetadata {
        exists: true,
        file_name,
        path: file_path.to_string_lossy().to_string(),
        modified_timestamp_ms: modified,
        export_date,
        deck_count,
        mastery_count,
    })
}

#[tauri::command]
fn read_subject_sync_file(app: tauri::AppHandle, file_name: String) -> Result<String, String> {
    let mut file_path = get_sync_dir(&app);
    file_path.push(&file_name);
    fs::read_to_string(&file_path).map_err(|e| e.to_string())
}

#[tauri::command]
fn save_subject_sync_file(app: tauri::AppHandle, file_name: String, content: String) -> Result<String, String> {
    let mut file_path = get_sync_dir(&app);
    file_path.push(&file_name);
    fs::write(&file_path, content).map_err(|e| e.to_string())?;
    Ok(file_path.to_string_lossy().to_string())
}

#[tauri::command]
fn open_url(url: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("rundll32")
            .args(["url.dll,FileProtocolHandler", &url])
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&url)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&url)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            open_sync_directory,
            check_subject_sync_status,
            read_subject_sync_file,
            save_subject_sync_file,
            open_url
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}