use libp2p::{identity, PeerId};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct DeRoulette {
    #[cfg(not(target_arch = "wasm32"))]
    allowed_to_close: bool,
    #[cfg(not(target_arch = "wasm32"))]
    show_confirmation_dialog: bool,
    name: String,
    text: String,
}

impl DeRoulette {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for DeRoulette {
    #[cfg(not(target_arch = "wasm32"))]
    fn on_close_event(&mut self) -> bool {
        self.show_confirmation_dialog = true;
        self.allowed_to_close
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::widgets::global_dark_light_mode_buttons(ui);
            ui.add(egui::TextEdit::multiline(&mut self.text));
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.name);
                if ui.button("Send").clicked() {
                    self.name = "".to_string();
                }
            });
            #[cfg(not(target_arch = "wasm32"))]
            if ui.button("Exit").clicked() {
                self.allowed_to_close = true;
                _frame.close();
            }
            if ui.button("Generate key").clicked() {
                let id_keys = identity::Keypair::generate_ed25519();
                let peer_id = PeerId::from(id_keys.public());
                self.text
                    .push_str(format!("Local peer id: {:?}\n", peer_id).as_str());
            }
        });
    }
}
