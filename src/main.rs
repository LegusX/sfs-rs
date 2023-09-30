#![windows_subsystem = "windows"]

use eframe::egui;
use egui::{ CentralPanel, TopBottomPanel, Window };
use egui_dnd::dnd;
use rfd::FileDialog;

mod config;
mod web;

fn main() {
    let mut config = config::new();
    let mut steam_closed = false;
    let mut steam_config_found = false;
    let mut users: Vec<web::User> = Default::default();

    //If the config path can be found, initiate the config immediately, if not, request for user to direct to it
    let config_path = match config::get_config_path() {
        Ok(r) => r,
        Err(_e) => String::from(""),
    };

    eframe
        ::run_simple_native("sfs-rs", Default::default(), move |ctx, _frame| {
            TopBottomPanel::top("header").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("sfs-rs");
                    ui.separator();
                });
            });
            //Popup warning user to close steam before continue
            if !steam_closed {
                Window::new("Please close Steam")
                    .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                    .resizable(false)
                    .collapsible(false)
                    .show(ctx, |ui| {
                        ui.label(
                            "Please fully close Steam before continuing\n(Make sure it has fully exited, and not just minimized to the system tray)"
                        );
                        if ui.button("Continue").clicked() {
                            steam_closed = true;
                        }
                    });
            } else if config_path.len() > 0 && !steam_config_found {
                match config.init(config_path.clone()) {
                    Err(e) => {
                        match e {
                            config::Error::NoAuthorizedDevice => {
                                new_error(
                                    "No AuthorizedDevice section found. Are you sure you have family sharing enabled?",
                                    &ctx
                                );
                            }
                            config::Error::ConfigNotFound => {} //Handled below
                        }
                    }
                    _ => {
                        steam_config_found = true;
                        users = match web::get_users(&config.vdf) {
                            Ok(users) => users,
                            Err(e) =>
                                match e {
                                    web::Error::InvalidID => {
                                        new_error("Invalid response from Steam API", &ctx);
                                        Default::default()
                                    }
                                    web::Error::RequestFailed => {
                                        new_error("API request failed", &ctx);
                                        Default::default()
                                    }
                                }
                        };
                    }
                }
            } else if !steam_config_found {
                Window::new("Can't find Steam config file")
                    .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                    .resizable(false)
                    .collapsible(false)
                    .show(ctx, |ui| {
                        ui.label(
                            "Steam config file could not be found. Please choose the file manually"
                        );
                        if ui.button("Choose file").clicked() {
                            if
                                let Some(path) = FileDialog::new()
                                    .add_filter("config", &["vdf"])
                                    .pick_file()
                            {
                                match config.init(path.to_string_lossy().to_string()) {
                                    Err(e) => {
                                        match e {
                                            config::Error::NoAuthorizedDevice => {
                                                new_error(
                                                    "No AuthorizedDevice section found. Are you sure you have family sharing enabled?",
                                                    &ctx
                                                );
                                            }
                                            config::Error::ConfigNotFound => {
                                                new_error("The file chosen does not exist", ctx)
                                            }
                                        }
                                    }
                                    Ok(()) => {
                                        steam_config_found = true;
                                        users = match web::get_users(&config.vdf) {
                                            Ok(users) => users,
                                            Err(e) =>
                                                match e {
                                                    web::Error::InvalidID => {
                                                        new_error(
                                                            "Invalid response from Steam API",
                                                            &ctx
                                                        );
                                                        Default::default()
                                                    }
                                                    web::Error::RequestFailed => {
                                                        new_error("API request failed", &ctx);
                                                        Default::default()
                                                    }
                                                }
                                        };
                                    }
                                };
                            }
                        }
                    });
            }

            CentralPanel::default().show(ctx, |ui| {
                //Drag and drop handler
                dnd(ui, "reorder_dnd").show_vec(&mut users, |ui, user, handle, _state| {
                    ui.horizontal(|ui| {
                        handle.ui(ui, |ui| {
                            ui.label(&user.personaname);
                        });
                    });
                });

                if ui.button("Save config").clicked() {
                    config.write();
                }
            });
        })
        .expect("Failed to start application!");
}

fn new_error(text: &str, ctx: &egui::Context) {
    Window::new("Error")
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .resizable(false)
        .collapsible(false)
        .show(ctx, |ui| { ui.label(text) });
}
