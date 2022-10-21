use crate::nbtween::{
    NbtFile,
};
use rfd;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct NbtweenApp {
    #[serde(skip)]
    files: Vec<NbtFile>,
    selected_file: Option<usize>,
}

impl Default for NbtweenApp {
    fn default() -> Self {
        Self {
            files: Vec::new(),
            selected_file: None,
        }
    }
}

impl NbtweenApp {
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

impl eframe::App for NbtweenApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { .. } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open NBT File").clicked() {
                        let paths = rfd::FileDialog::new()
                            .set_directory("/")
                            .pick_files();
                        if let Some(files) = paths {
                            self.files.extend(
                                files.iter()
                                    .map(|path| NbtFile::load(path))
                                    .filter_map(|file| file.ok())
                            );
                        }
                    }
                    #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!=
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        // egui::SidePanel::left("side_panel").show(ctx, |ui| {
            
        // });

        egui::CentralPanel::default().show(ctx, |ui| {
            // This is where the main NBT editor will be built.
            let filecount = self.files.len().to_string();
            ui.label(filecount);
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
    }
}
