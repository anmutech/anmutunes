use crate::defs::{ConfigState, Language, Theme, ThemeColors, Version, View};
use directories::UserDirs;
use log::{debug, error};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

// TODO: implement upgrade mechanism, if more entries are added to ConfigState
// We could try to load the existing json file using serde_json but into an undefined struct
// Then test if any known fields are included.
// If some are, copy their values over and insert defaults for the other fields.
// We should inform the user that their config was changed in this event...
// Probably best is to have versioning in the config.json with a version field.

pub fn load_or_setup_config_path(app: AppHandle) -> Result<PathBuf, String> {
    // Get the app_config_dir to load the sqlite db from
    match app.path().app_config_dir() {
        Ok(config_dir) => {
            debug!("{:?}", config_dir);
            if !config_dir.exists() {
                match fs::create_dir_all(&config_dir) {
                    Ok(()) => {}
                    Err(error) => {
                        error!(
                            "Could not create app config directory. {}",
                            error.to_string()
                        );
                        return Err(format!(
                            "Could not create app config directory. {}",
                            error.to_string()
                        ));
                    }
                }
                debug!("Directory created: {:?}", config_dir);
            }

            return Ok(config_dir);
        }
        Err(error) => {
            error!("Could not get app config directory. {}", error.to_string());
            return Err(format!(
                "Could not get app config directory. {}",
                error.to_string()
            ));
        }
    }
}

pub fn get_config(mut config_path: PathBuf) -> Result<ConfigState, String> {
    if tauri::is_dev() {
        config_path.push("debug.config.json");
    } else {
        config_path.push("config.json");
    }

    match File::open(&config_path) {
        Ok(mut config_file) => {
            let mut config_json = "".to_string();

            match config_file.read_to_string(&mut config_json) {
                Ok(_) => match serde_json::from_str::<ConfigState>(&config_json) {
                    Ok(config_state) => {
                        return Ok(config_state);
                    }
                    Err(error) => {
                        error!("{}", error);
                        return Err(format!(
                            "{:?} is not a valid config.json for anmutunes",
                            config_path
                        ));
                    }
                },
                Err(error) => {
                    error!("{}", error);
                    return Err(format!("Could not read file: {:?}", config_path));
                }
            }
        }
        Err(error) => {
            error!("{}", error);
            return Err(format!("Could not open file: {:?}", config_path));
        }
    }
}

pub fn create_config(mut config_path: PathBuf) -> Result<ConfigState, String> {
    if tauri::is_dev() {
        config_path.push("debug.config.json");
    } else {
        config_path.push("config.json");
    }

    let default_config = ConfigState {
        version: Version {
            major: 1,
            minor: 0,
            patch: 0,
        },
        theme: Theme::System,
        custom_colors: ThemeColors {
            ..Default::default()
        },
        startup_view: View::Recents,
        language: Language::System,
        look_for_updates: true,
        media_path: if let Some(user_dirs) = UserDirs::new() {
            match user_dirs.audio_dir() {
                Some(audio_dir) => match audio_dir.to_string_lossy() {
                    std::borrow::Cow::Borrowed(dir) => dir.to_string(),
                    std::borrow::Cow::Owned(dir) => dir,
                },
                None => "".to_string(),
            }
        } else {
            "".to_string()
        },
        manage_folders: false,
        allow_delete_from_db: false,
        allow_delete_files: false,
        is_new: true,
    };

    match serde_json::to_string(&default_config) {
        Ok(config_json) => match File::create(&config_path) {
            Ok(mut config_file) => match config_file.write_all(config_json.as_bytes()) {
                Ok(()) => return Ok(default_config),
                Err(_) => return Err(format!("Could not write to file: {:?}", config_path)),
            },
            Err(error) => {
                error!("{}", error);
                return Err(format!("Could not create file: {:?}", config_path));
            }
        },
        Err(error) => {
            error!("{}", error);
            return Err(format!("Could not create configuration json string."));
        }
    }
}

pub fn set_config(mut config_path: PathBuf, config: ConfigState) -> Result<ConfigState, String> {
    if tauri::is_dev() {
        config_path.push("debug.config.json"); // test
    } else {
        config_path.push("config.json"); // release
    }

    match serde_json::to_string(&config) {
        Ok(config_json) => match File::options()
            .write(true)
            .truncate(true)
            .open(&config_path)
        {
            Ok(mut config_file) => match config_file.write_all(config_json.as_bytes()) {
                Ok(()) => return Ok(config),
                Err(error) => {
                    error!("{}", error);
                    return Err(format!("Could not write to file: {:?}", config_path));
                }
            },
            Err(error) => {
                error!("{}", error);
                return Err(format!("Could not open file: {:?}", config_path));
            }
        },
        Err(error) => {
            error!("{}", error);
            return Err(format!("Could not create configuration json string."));
        }
    }
}
