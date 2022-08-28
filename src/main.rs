#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Confirm exit",
        options,
        Box::new(|_cc| Box::new(DeRoulette::default())),
    );
}

#[derive(Default)]
struct DeRoulette {
    allowed_to_close: bool,
    show_confirmation_dialog: bool,
    name: String,
}

impl eframe::App for DeRoulette {
    fn on_close_event(&mut self) -> bool {
        self.show_confirmation_dialog = true;
        self.allowed_to_close
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::widgets::global_dark_light_mode_buttons(ui);
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
                if ui.button("Send").clicked() {
                    self.name = "".to_string();
                }
            });
            if ui.button("Exit").clicked() {
                self.allowed_to_close = true;
                frame.close();
            }
        });
    }
}
