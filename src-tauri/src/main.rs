// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::api::process::{Command as Cmd, CommandEvent};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn generate_backup(window: tauri::Window, url: &str, folder: &str) -> String {
    let cmd = Cmd::new_sidecar("db-dump").unwrap();
    dbg!(&cmd);
    let cmd = cmd.args([url, "-o", folder.clone()]);
    dbg!(&cmd);
    let (mut rx, mut child) = cmd.spawn().unwrap();

    tauri::async_runtime::spawn(async move {
        // read events such as stdout
        while let Some(event) = rx.recv().await {
            dbg!(&event);
            if let CommandEvent::Stderr(line) = event {
                dbg!(line);
                // write to stdin
                child.write("message from Rust\n".as_bytes()).unwrap();
            }
        }
        window
            .emit("status", format!("your db has backuped"))
            .unwrap();
        println!("run once");
    });
    format!("Done")
}

#[tauri::command]
fn restore_db(window: tauri::Window, url: &str, path: &str) -> String {
    let cmd = Cmd::new_sidecar("db-restore").unwrap();
    dbg!(&cmd);
    let cmd = cmd.args([url, path]);
    dbg!(&cmd);
    let (mut rx, mut child) = cmd.spawn().unwrap();

    tauri::async_runtime::spawn(async move {
        // read events such as stdout
        while let Some(event) = rx.recv().await {
            dbg!(&event);
            if let CommandEvent::Stderr(line) = event {
                dbg!(line);
                // write to stdin
                child.write("message from Rust\n".as_bytes()).unwrap();
            }
        }
        window
            .emit("status", format!("your db has restored"))
            .unwrap();
        println!("run once");
    });
    format!("Done")
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate_backup, restore_db])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
