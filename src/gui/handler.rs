use eframe::App;
use egui::{Layout, ScrollArea};
use log::info;

pub struct Handler {
    pub music_locations: Vec<String>,
    pub picked_path: Option<String>,
}

impl Handler {
    pub fn new() -> Self {
        Self {
            music_locations: vec![
                "a".to_owned(),
                "b".to_owned(),
            ],
            picked_path: None,
        }
    }
}

impl App for Handler {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Open directory...").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.picked_path = Some(path.display().to_string());
                }
            }
            
            if self.picked_path.is_some() {
                ui.columns(2, |cols| {
                    cols[0].with_layout(Layout::right_to_left(egui::Align::Min),
                    |ui| {
                        ScrollArea::vertical().id_source("left_scroll").show(ui, |ui| {
                            ui.vertical(|ui| {
                                self.show_available_music(ui);
                            });
                        });
                    });
                    
                    cols[1].with_layout(Layout::right_to_left(egui::Align::Min), 
                    |ui| {
                        ScrollArea::vertical().id_source("right_scroll").show(ui, |ui| {
                            ui.vertical(|ui| {
                                for music in &self.music_locations {
                                    ui.label(music);
                                    ui.add_space(1.);
                                }
                            });
                        });
                    });
                });
            }
        });
    }
}

impl Handler {
    fn show_available_music(&mut self, ui:&mut egui::Ui) {
        if let Some(picked_path) = &self.picked_path {
            for entry in glob::glob(&format!("{}/*.mp3", picked_path))
                    .expect("Failed to read glob pattern") {
                let file = entry.unwrap();
                let file_name = file.file_name().unwrap().to_str().unwrap();
                if ui.button(file_name).clicked() {
                    info!("file {} added to playlist", file_name);
                    self.music_locations.push(file_name.to_owned());
                    info!("current list size: {}", self.music_locations.len());
                };
            }
            for entry in glob::glob(&format!("{}/*.ncm", picked_path))
                    .expect("Failed to read glob pattern") {
                let file = entry.unwrap();
                let file_name = file.file_name().unwrap().to_str().unwrap();
                if ui.label(file_name).clicked() {
                    self.music_locations.push(file_name.to_owned());
                };
            }
        }
    }
}