#![windows_subsystem = "windows"]

use eframe::egui;
use egui::{ CentralPanel, TopBottomPanel, Window };
use egui_dnd::dnd;
use rfd::FileDialog;

mod config;

fn main() {
    let mut config = config::new();
    let mut steam_closed = false;
    let mut steam_config_found = false;

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
            } else if config_path.len() > 0 {
                match config.init(config_path.clone()) {
                    Err(e) => {
                        match e {
                            config::Error::NoAuthorizedDevice => {
                                Window::new("Error:")
                                    .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                                    .resizable(false)
                                    .collapsible(false)
                                    .show(ctx, |ui| {
                                        ui.label(
                                            "Failed to find any accounts connected to Family Sharing."
                                        )
                                    });
                            }
                            config::Error::ConfigNotFound => {} //Handled below
                        }
                    }
                    _ => {
                        steam_config_found = true;
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
                                steam_config_found = true;
                                match config.init(path.to_string_lossy().to_string()) {
                                    Err(e) => {
                                        match e {
                                            _ => {
                                                Window::new("Hello world").show(ctx, |ui| {
                                                    ui.label("Hi");
                                                });
                                            }
                                        }
                                    }
                                    _ => {}
                                };
                            }
                        }
                    });
            }
            CentralPanel::default().show(ctx, |ui| {
                //Drag and drop handler
                // println!("{:?}", &config.vdf);
                dnd(ui, "reorder_dnd").show_vec(&mut config.vdf, |ui, item, handle, _state| {
                    ui.horizontal(|ui| {
                        handle.ui(ui, |ui| {
                            ui.label(&item.id);
                        });
                    });
                });

                if ui.button("Save config").clicked() {
                    // config.write();
                    println!("{:?}", config.vdf.pop());
                }
            });
        })
        .expect("Failed to start application!");
}
