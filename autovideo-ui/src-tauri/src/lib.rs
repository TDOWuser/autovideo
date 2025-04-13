#![allow(clippy::too_many_arguments)]

use std::path::PathBuf;
use autovideo_core::{process_videos, Mode, ScriptInfo};
use serde::Serialize;
use tauri::{Window, Emitter};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[derive(Clone, Serialize)]
struct Progress {
    current: usize,
    max: usize,
}

#[tauri::command]
async fn convert_files(
    window: Window,
    inputs: Vec<PathBuf>,
    input_esp: Option<PathBuf>,
    input_esp_drive_in: Option<PathBuf>,
    mod_name: String,
    input_framerate: u32,
    short_names: bool,
    video_name: Option<String>,
    size: u32,
    keep_aspect_ratio: bool,
    script_info: Option<ScriptInfo>,
) -> Result<(), String> {
    let mut progress = Progress {
        current: 0,
        max: if keep_aspect_ratio {
            inputs.len() * 3
        } else {
            inputs.len() * 2
        },
    };
    
    process_videos(
        inputs,
        input_esp,
        input_esp_drive_in,
        mod_name,
        input_framerate,
        short_names,
        video_name,
        size,
        keep_aspect_ratio,
        script_info.is_some(),
        script_info,
        Mode::UiMode,
        || {
            progress.current += 1;
            window.emit("listener", progress.clone()).unwrap();
        }
    )?;
    
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![convert_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
