use eframe::egui;
use egui::{ CentralPanel, TopBottomPanel, Window };
use egui_dnd::dnd;

mod config;

fn main() {
    let mut config = config::new();
    let mut steam_closed = false;

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
            }
            CentralPanel::default().show(ctx, |ui| {
                //Drag and drop handler
                dnd(ui, "reorder_dnd").show_vec(&mut config.vdf, |ui, item, handle, _state| {
                    ui.horizontal(|ui| {
                        handle.ui(ui, |ui| {
                            ui.label(&item.id);
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
