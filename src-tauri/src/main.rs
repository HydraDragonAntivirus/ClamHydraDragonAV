#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::api::{dialog::FileDialogBuilder, path::home_dir};

#[derive(serde::Deserialize)]
struct SelectFolder {}

#[tauri::command]
async fn select_folder(_command: SelectFolder) -> tauri::Result<String> {
    let home_dir = home_dir().unwrap();
    let dialog = FileDialogBuilder::new()
        .set_directory(&home_dir);
    dialog.pick_folder(|result| {
        match result {
            Some(_path) => { /* handle the selected path */ },
            None => { /* handle the case where no folder was selected */ },
        }
    });
    Ok("Folder selected".to_string()) // Replace this with your own logic
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![select_folder])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
