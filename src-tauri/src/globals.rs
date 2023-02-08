use log::{trace, error, info};
use serde::{Deserialize, Serialize};
use tauri::Manager;
use std::fmt;

use std::path::PathBuf;
use std::sync::Mutex;
use directories::ProjectDirs;

#[derive(Debug, Clone)]
pub struct Window {
    pub window: tauri::Window
}

pub static MAIN_WINDOW: Mutex<Option<Window>> = Mutex::new(None);
pub static CONFIG: Mutex<Option<ConfigTemplate>> = Mutex::new(None);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigTemplate {
}

impl fmt::Display for ConfigTemplate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

lazy_static! {
    pub static ref CONFIG_LOCATION: PathBuf = {
        let dirs = ProjectDirs::from("dev", "", "NEAV1E_rust");
        match dirs {
        Some(_) => return dirs.unwrap().config_dir().to_path_buf().join("config.json"),
        None => todo!(),
        }
    };
}

pub fn get_main_window() -> tauri::Window {
    trace!("getting current window reference");

    match MAIN_WINDOW.lock().unwrap().clone() {
        Some(mw) => {
            trace!("window name: {}", mw.window.label());
            mw.window
        },
        None => {
            error!("Failed to get main window!");
            panic!("Failed to get main window!");
        },
    }
}

pub fn set_main_window(new_window: tauri::Window) {
    trace!("Setting main Window: {}", new_window.label());
    *MAIN_WINDOW.lock().unwrap() = Some(Window { window: new_window });
}

pub fn update_frontend() {
    info!("Updating frontend!");
    get_main_window().emit_all("update_frontend", true).expect("Error Sending directory changed event to frontend!");
}