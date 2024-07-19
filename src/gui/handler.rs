use std::sync::Arc;

use eframe::App;
use egui::mutex::RwLock;

use super::file_dialog::DialogHandler;

pub struct Handler {
    pub play_list: Vec<String>,
    pub picked_path: Arc<RwLock<Option<String>>>,
    pub dialog_handler: DialogHandler,
    pub dump_mode: bool,
}

impl Handler {
    pub fn new() -> Self {
        let picked_path = Arc::new(RwLock::new(None));
        Self {
            play_list: vec![],
            picked_path: picked_path.clone(),
            dialog_handler: DialogHandler::new(picked_path),
            dump_mode: false,
        }
    }
}

impl App for Handler {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.dialog_handler.update_list();
        egui::CentralPanel::default().show(ctx, |ui| {
            self.update_picked_path(ui);
            self.display_selectable_music(ui);
        });
    }
}

impl Handler {
    fn update_picked_path(&self, ui: &mut egui::Ui) {
        if ui.button("Open directory...").clicked() {
            if let Some(path) = rfd::FileDialog::new().pick_folder() {
                let mut path_task = self.picked_path.write();
                path_task.replace(path.to_str().unwrap().to_string());
            }
        }
    }

    fn display_selectable_music(&self, ui:&mut egui::Ui) {
        let list_ref = self.dialog_handler.list_ref();
        for music_path in list_ref.read().iter() {

        }
    }
}