mod audio;
mod config;
mod db;
mod defs;

extern crate vlc;
use crate::audio::create_player_instance;
use crate::config::{create_config, get_config, load_or_setup_config_path, set_config};
use crate::db::create_sqlite_instance;
use crate::defs::{AudioRequest, ConfigRequest, DBData, DBRequest};
use log::{debug, error};
use std::sync::Mutex;
use std::sync::{mpsc, mpsc::Sender};
use tauri::{AppHandle, Emitter};

#[tauri::command]
fn audiorequest(audio_sender: tauri::State<Mutex<Sender<AudioRequest>>>, request: AudioRequest) {
    debug!("audiorequest {:?}", request.clone());
    match audio_sender.lock() {
        Ok(audio_lock) => {
            match audio_lock.send(request) {
                Ok(()) => {}
                Err(error) => {
                    error!("{}", error);
                    // TODO: send error to frontend?
                }
            }
        }
        Err(error) => {
            error!("{}", error);
            // TODO: send error to frontend?
        }
    }
}

#[tauri::command]
fn dbrequest(db_sender: tauri::State<Mutex<Sender<DBRequest>>>, request: DBRequest) {
    debug!("dbrequest {:?}", request.clone());
    match db_sender.lock() {
        Ok(db_lock) => {
            match db_lock.send(request) {
                Ok(()) => {}
                Err(error) => {
                    error!("{}", error);
                    // TODO: send error to frontend?
                }
            }
        }
        Err(error) => {
            error!("{}", error);
            // TODO: send error to frontend?
        }
    }
}

#[tauri::command]
fn configrequest(
    app: AppHandle,
    db_sender: tauri::State<Mutex<Sender<DBRequest>>>,
    request: ConfigRequest,
) {
    debug!("configrequest {:?}", request.clone());
    match load_or_setup_config_path(app.clone()) {
        Ok(config_path) => match request {
            ConfigRequest::Get => {
                match get_config(config_path) {
                    Ok(config) => {
                        debug!("got config {:?}", config);
                        match app.emit("config_state", config) {
                            Ok(()) => {}
                            Err(error) => {
                                error!("{}", error);
                                // TODO: send error to frontend?
                            }
                        };
                    }
                    Err(error) => {
                        error!("{}", error);
                        // TODO: send error to frontend?
                    }
                }
            }
            ConfigRequest::Set(config) => {
                // send DBRequest if relevant changes to (media path, allow delete from db, allow delete files, manage_folders)
                match set_config(config_path, config) {
                    Ok(new_config) => {
                        match db_sender.lock() {
                            Ok(db_lock) => {
                                match db_lock.send(DBRequest::UpdateConfig(new_config.clone())) {
                                    Ok(()) => {}
                                    Err(error) => {
                                        error!("{}", error);
                                        // TODO: send error to frontend?
                                    }
                                }
                            }
                            Err(error) => {
                                error!("{}", error);
                                // TODO: send error to frontend?
                            }
                        }
                        match app.emit("config_state", new_config) {
                            Ok(()) => {}
                            Err(error) => {
                                error!("{}", error);
                                // TODO: send error to frontend?
                            }
                        };
                    }
                    Err(error) => {
                        error!("{}", error);
                        // TODO: emit error event
                        //app.emit(event, payload);
                    }
                }
            }
        },
        Err(error) => {
            error!("{}", error);
            // TODO: emit error event
            //app.emit(event, payload);
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let tauri_logger = if tauri::is_dev() {
        tauri_plugin_log::Builder::new()
            .level(log::LevelFilter::Debug)
            .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepSome(5))
            .build()
    } else {
        tauri_plugin_log::Builder::new()
            .level(log::LevelFilter::Info)
            .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepSome(5))
            .build()
    };

    // Create channels for communication between threads
    let (audio_sender, audio_receiver) = mpsc::channel::<AudioRequest>();
    let (db_sender, db_receiver) = mpsc::channel::<DBRequest>();
    let (data_sender, data_receiver) = mpsc::channel::<DBData>();
    let db_sender_audio = db_sender.clone();

    // Create builder and add logger
    let mut builder = tauri::Builder::default();
    builder = builder.plugin(tauri_logger);

    #[cfg(not(target_os = "linux"))]
    {
        // This works great in dev, but fails in production on Gnome...
        // Fractional scaling causes additional issue of window getting smaller on reopen
        builder = builder.plugin(tauri_plugin_window_state::Builder::new().build());
    }

    builder
        .plugin(tauri_plugin_single_instance::init(|_app, _args, _cwd| {}))
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_os::init())
        .manage(Mutex::new(audio_sender.clone()))
        .manage(Mutex::new(db_sender))
        .setup(|app| {
            match load_or_setup_config_path(app.handle().clone()) {
                Ok(config_path) => {
                    match get_config(config_path.clone()) {
                        Ok(config_state) => {
                            create_sqlite_instance(
                                app.handle().clone(),
                                db_receiver,
                                data_sender,
                                config_state,
                            );
                            create_player_instance(
                                app.handle().clone(),
                                audio_receiver,
                                data_receiver,
                                db_sender_audio,
                            );
                        }
                        Err(error) => {
                            // TODO: notify user that config has been reset to default
                            // TODO: move old config to config.json.bkp
                            println!("{}", error);
                            match create_config(config_path.clone()) {
                                Ok(config_state) => {
                                    create_player_instance(
                                        app.handle().clone(),
                                        audio_receiver,
                                        data_receiver,
                                        db_sender_audio,
                                    );
                                    create_sqlite_instance(
                                        app.handle().clone(),
                                        db_receiver,
                                        data_sender,
                                        config_state,
                                    );
                                }
                                Err(error) => {
                                    error!("{}", error);
                                    // TODO: setup such that all the error messages are sent to frontend on any request? Probably overkill
                                }
                            }
                        }
                    }
                }
                Err(error) => {
                    error!("{}", error.to_string());
                }
            }

            return Ok(());
        })
        .invoke_handler(tauri::generate_handler![
            configrequest,
            audiorequest,
            dbrequest
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
