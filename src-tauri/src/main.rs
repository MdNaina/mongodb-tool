// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::process::Command;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn generate_backup(url: &str, folder: &str, db: &str) -> String {
    let cmd = Command::new("mongodump")
        .args(["--uri", url])
        .arg(format!("--db={}", db))
        .args(["-o", folder])
        // .arg(format!("--archive={}/{}.db", folder, db))
        // .arg("--gzip")
        .output()
        .expect("cannot open");
    format!("DB log:\n{}", String::from_utf8_lossy(&cmd.stderr))
}

#[tauri::command]
fn restore_db(url: &str, db: &str, path: &str) -> String {
    let cmd = Command::new("mongorestore")
        .arg(url)
        .arg(format!("--nsInclude={}", db))
        // .arg(format!("--archive={}", path))
        .arg(path)
        .output()
        .expect("cannot open");
    format!(
        "DB log {}:\n {}",
        &cmd.status,
        String::from_utf8_lossy(&cmd.stderr)
    )
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate_backup, restore_db])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
