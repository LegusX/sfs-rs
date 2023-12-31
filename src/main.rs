#![cfg_attr(all(target_os = "windows", not(debug_assertions)), windows_subsystem = "windows")]

use eframe::{ egui, NativeOptions };
use egui::{ CentralPanel, TopBottomPanel, Window, vec2, ScrollArea };
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

    let mut eframe_options: NativeOptions = Default::default();
    eframe_options.initial_window_size = Some(vec2(300.0, 500.0));

    eframe
        ::run_simple_native("sfs-rs", eframe_options, move |ctx, _frame| {
            egui_extras::install_image_loaders(ctx);
            TopBottomPanel::top("header").show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("sfs-rs");
                    // ui.separator();
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
            } else if
                // Triggers when we know the config file exists, but haven't yet loaded it
                config_path.len() > 0 &&
                !steam_config_found
            {
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
                        println!("{:?}", users);
                    }
                }
            } else if
                // Triggers when we were unable to find the config file, and haven't loaded it yet
                !steam_config_found
            {
                Window::new("Can't find Steam config file")
                    .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                    .resizable(false)
                    .collapsible(false)
                    .show(ctx, |ui| {
                        ui.label(
                            "Steam config file could not be found. Please choose the file manually"
                        );

                        // Bring up file dialog so user can manually select config.vdf
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
            if users.len() > 0 {
                CentralPanel::default().show(ctx, |ui| {
                    ui.style_mut().spacing.item_spacing = vec2(10.0, 15.0);

                    //Drag and drop handler
                    ScrollArea::vertical().show(ui, |ui| {
                        dnd(ui, "reorder_dnd").show_vec(&mut users, |ui, user, handle, _state| {
                            ui.horizontal(|ui| {
                                handle.ui(ui, |ui| {
                                    ui.add(
                                        egui::Image
                                            ::from_uri(&user.uri)
                                            .maintain_aspect_ratio(true)
                                            .fit_to_exact_size(vec2(32.0, 32.0))
                                            .rounding(5.0)
                                    );
                                    ui.label(&user.personaname);
                                });
                            });
                        });
                    });

                    ui.vertical_centered(|ui| {
                        if ui.button("Save config").clicked() {
                            config.write();
                        }
                    });
                    ui.separator();
                    ui.heading("Instructions:");
                    ui.label(
                        "Drag and drop the users above to rearrange their priority for Steam Family Sharing. Users on top will have their libraries borrowed from first."
                    )
                });
            }
        })
        .expect("Failed to start application!");
}

// Create a basic error popup
fn new_error(text: &str, ctx: &egui::Context) {
    Window::new("Error")
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .resizable(false)
        .collapsible(false)
        .show(ctx, |ui| { ui.label(text) });
}
