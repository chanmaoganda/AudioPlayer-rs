use std::path::PathBuf;

use eframe::App;

pub struct Handler {
    pub music_locations: Vec<PathBuf>,
    pub picked_path: Option<String>,
}

impl Handler {
    pub fn new() -> Self {
        Self {
            music_locations: vec![],
            picked_path: None,
        }
    }
}

impl App for Handler {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Open directory...").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.picked_path = Some(path.display().to_string());
                }
            }
    
            if let Some(picked_path) = &self.picked_path {
                ui.horizontal(|ui| {
                    ui.label("Picked Dir:");
                    ui.monospace(picked_path);
                });
            }
        });
    }
}