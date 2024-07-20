use std::{path::PathBuf, sync::{mpsc::SyncSender, Arc}, thread::{self}, time::Duration};

use eframe::App;
use egui::{mutex::RwLock, Color32, FontDefinitions};

use crate::event::Event;

use super::file_dialog::DialogHandler;

pub struct Handler {
    pub play_list: Arc<RwLock<Vec<PathBuf>>>,
    pub picked_path: Arc<RwLock<Option<String>>>,
    pub dialog_handler: DialogHandler,
    pub dump_mode: bool,
    pub sender: SyncSender<Event>,
}

impl Handler {
    pub fn new(cc: &eframe::CreationContext, sender: SyncSender<Event>) -> Self {
        Self::customize_font(&cc.egui_ctx);
        let picked_path = Arc::new(RwLock::new(None));
        Self {
            play_list: Arc::new(RwLock::new(Vec::new())),
            picked_path: picked_path.clone(),
            dialog_handler: DialogHandler::new(picked_path),
            dump_mode: false,
            sender,
        }
    }
}

impl App for Handler {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        thread::sleep(Duration::from_millis(5));
        self.set_font_style(ctx);
        self.dialog_handler.update_list();
        egui::CentralPanel::default().show(ctx, |ui| {
            self.update_picked_path(ui);
            self.display_selectable_music(ui);
        });
    }
}

impl Handler {
    fn customize_font(ctx: &egui::Context) {
        let mut font = FontDefinitions::default();
        font.font_data.insert(
            "MesloLGS".to_string(),
            egui::FontData::from_static(include_bytes!("../../assets/font/MesloLGS_NF_Regular.ttf"))
        );
        font.families.entry(
            egui::FontFamily::Proportional).or_default()
            .insert(0, "MesloLGS".to_owned());
        font.families.entry(
            egui::FontFamily::Monospace).or_default()
            .insert(0, "MesloLGS".to_owned());
        ctx.set_fonts(font);
    }

    fn set_font_style(&self, ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();
        style.text_styles = [
            (egui::TextStyle::Heading, egui::FontId::new(12.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Body, egui::FontId::new(11.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Monospace, egui::FontId::new(8.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Button, egui::FontId::new(14.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Small, egui::FontId::new(10.0, egui::FontFamily::Proportional)),
        ]
        .into();
        ctx.set_style(style);
    }

    fn update_picked_path(&self, ui: &mut egui::Ui) {
        if ui.button("Open directory...").clicked() {
            if let Some(path) = rfd::FileDialog::new().pick_folder() {
                let mut path_task = self.picked_path.write();
                path_task.replace(path.to_str().unwrap().to_string());
                log::info!("Picked path: {}", path.to_str().unwrap());
            }
        }
    }

    fn display_selectable_music(&self, ui:&mut egui::Ui) {
        let list_ref = self.dialog_handler.list_ref();
        ui.vertical(|ui| {
            list_ref.write().iter().for_each(|path| {
                self.display_music_bar(ui, path);
            });
        });
    }

    fn display_music_bar(&self, ui: &mut egui::Ui, path: &PathBuf) {
        let name = path.file_name().unwrap().to_str().unwrap();
        ui.horizontal(|ui| {
            ui.add_space(2.);
            ui.colored_label(Color32::BROWN, name);
            ui.add_space(10.);

            if ui.button("⏸").clicked() {
                self.sender.send(Event::Pause).unwrap();
            }
            if ui.button("▶").clicked() {
                self.sender.send(Event::Next(path.clone())).unwrap();
            }
            if ui.button("+").clicked() {
                let mut write_task = self.play_list.write();
                write_task.push(path.clone());
                self.sender.send(Event::Append(path.clone())).unwrap();
            }

            if ui.button("🔁").clicked() {

            }
            if ui.button("🔀").clicked() {

            }

        });
    }
}

impl Handler {

}