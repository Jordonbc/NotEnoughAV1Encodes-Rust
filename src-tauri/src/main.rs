#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate lazy_static;

use log::{info, trace};
use log4rs;
use std::process::Command;
use std::env;
use tauri::{Manager, Size, LogicalSize};

use crate::globals::*;
mod globals;
mod tauri_commands;
use crate::config::*;
mod config;

fn kill_process_tree(pid: u32) {
    if cfg!(target_os = "windows") {
        info!("Killing via taskkill...");
        Command::new("taskkill")
            .args(["/pid", &pid.to_string(), "/T", "/F"])
            .output()
            .expect("Failed to kill process tree");
    } else {
        info!("Killing via pkill...");
        Command::new("pkill")
            .arg("-P")
            .arg(pid.to_string())
            .output()
            .expect("Failed to kill process tree");
    }
}

fn main() {
    log4rs::init_file("logging_config.yaml", Default::default()).unwrap();
    trace!("Hello, World! I'm awake!");
    read_config_file();
    
    tauri::Builder::default()
        .setup(|app|{
            set_main_window(app.get_window("main").unwrap());

            get_main_window().set_size(Size::Logical(LogicalSize { width: 1085.0, height: 650.0 })).expect("Error setting window size!");
            get_main_window().set_min_size(Some(LogicalSize { width: 780.0, height: 550.0 })).expect("Failed to set min size");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    
}
