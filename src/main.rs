#![windows_subsystem = "windows"]

use std::{fs, ops::Not};

use anyhow::anyhow;
use dirs::data_dir;
use iced::advanced::Application;
use pumpbin::{error_dialog, plugin::CONFIG_FILE_PATH, settings, Pumpbin};

fn main() {
    match try_main() {
        Ok(_) => (),
        Err(e) => {
            error_dialog(e);
        }
    }
}

fn try_main() -> anyhow::Result<()> {
    let mut config_path = data_dir().ok_or(anyhow!("Get data_dir failed."))?;
    config_path.push("PumpBin");
    config_path.push("plugins");

    if let Some(parent) = config_path.parent() {
        if parent.exists().not() {
            fs::create_dir_all(parent)?;
        } else if parent.is_dir().not() {
            fs::remove_file(parent)?;
            fs::create_dir_all(parent)?;
        }
    }

    CONFIG_FILE_PATH
        .set(config_path)
        .map_err(|_| anyhow!("Set CONFIG_FILE_PATH failed."))?;

    Pumpbin::run(settings())?;
    Ok(())
}
