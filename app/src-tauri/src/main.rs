// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod pc_settings;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use std::env;

use sysinfo::{DiskExt, System, SystemExt};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return;
    }

    if args[1] != "SpecialArg" {
        return;
    }

    let sys = System::new_all();

    let settings = pc_settings::Settings::deser("/home/karim/pc_settings.json").unwrap();

    let disks: Vec<String> = sys
        .disks()
        .iter()
        .map(|x| x.name().to_string_lossy().to_string())
        .collect();
    let name = sys.host_name().unwrap();

    if name != settings.host_name || disks != settings.disk {
        return;
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
