use std::{path::PathBuf, sync::{mpsc::{Receiver, SyncSender}, Arc}, thread::{self}, time::Duration};

use eframe::App;
use egui::{mutex::RwLock, Color32, FontDefinitions};

use crate::{event::Event, player::parser};

use super::file_dialog::DialogHandler;

pub struct PlayerConfig {
    pub dump_mode: bool,
    pub current_music: Option<PathBuf>,
    pub current_pos: u64,
    pub is_playing: bool,
}

pub struct Handler {
    pub play_list: Arc<RwLock<Vec<PathBuf>>>,
    pub picked_path: Arc<RwLock<Option<String>>>,
    pub dialog_handler: DialogHandler,
    pub event_sender: SyncSender<Event>,
    pub pos_receiver: Receiver<Duration>,
    pub config: PlayerConfig,
}

impl Handler {
    pub fn new(cc: &eframe::CreationContext, event_sender: SyncSender<Event>, pos_receiver: Receiver<Duration>) -> Self {
        Self::customize_font(&cc.egui_ctx);
        let picked_path = Arc::new(RwLock::new(None));
        Self {
            play_list: Arc::new(RwLock::new(Vec::new())),
            picked_path: picked_path.clone(),
            dialog_handler: DialogHandler::new(picked_path),
            event_sender,
            pos_receiver,
            config: PlayerConfig {
                current_music: None,
                current_pos: 0,
                dump_mode: false,
                is_playing: false,
            }
        }
    }
}

impl App for Handler {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        thread::sleep(Duration::from_millis(5));
        match self.pos_receiver.try_recv() {
            Ok(pos) => {
                self.config.current_pos = pos.as_secs();
            }
            Err(_) => {}
        }
        self.set_font_style(ctx);
        self.dialog_handler.update_list();
        if !self.config.dump_mode {
            egui::CentralPanel::default().show(ctx, |ui| {
                self.update_picked_path(ui);
                self.display_selectable_music(ui);
            });
            egui::TopBottomPanel::bottom("bar").show(ctx, |ui| {
                let music = self.config.current_music.clone();
                self.display_progress_bar(ui, music);
            });
        } else {
            egui::CentralPanel::default().show(ctx, |ui| {
                self.display_dump_mode(ui);
            });
        }
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

    fn display_selectable_music(&mut self, ui:&mut egui::Ui) {
        let list_ref = self.dialog_handler.list_ref();
        ui.vertical(|ui| {
            list_ref.read().iter().for_each(|path| {
                self.display_music_bar(ui, path);
            });
        });
    }

    fn display_music_bar(&mut self, ui: &mut egui::Ui, path: &PathBuf) {
        let name = path.file_name().unwrap().to_str().unwrap();
        ui.horizontal(|ui| {
            ui.add_space(2.);
            ui.colored_label(Color32::BROWN, name);
            ui.add_space(10.);

            if ui.button("▶").clicked() {
                self.config.is_playing = true;
                self.config.current_music = Some(path.clone());
                self.event_sender.send(Event::Next(path.clone())).unwrap();
            }
            if ui.button("+").clicked() {
                let mut write_task = self.play_list.write();
                write_task.push(path.clone());
                self.event_sender.send(Event::Append(path.clone())).unwrap();
            }
            // TODO: care about update current_music!
            // if ui.button("🔁").clicked() {

            // }
            // if ui.button("🔀").clicked() {

            // }

        });
    }

    fn display_progress_bar(&mut self, ui: &mut egui::Ui, music: Option<PathBuf>) {

        if music.is_none() {
            ui.add(egui::Slider::new(&mut self.config.current_pos, 0..=100));
            return;
        }
        
        let duration = parser::get_duration(music.as_ref().unwrap()).unwrap();
        let bar = egui::ProgressBar::new(self.config.current_pos as f32 / duration.as_secs_f32())
            .desired_width(300.);
        ui.add(bar);
        ui.label(format!("{} / {}", display_time(&self.config.current_pos), display_time(&duration.as_secs())));   
        ui.horizontal(|ui| {
            if ui.button("⏮").on_hover_text("Rewind 5 seconds").clicked() {
                self.event_sender.send(Event::Rewind5s).unwrap();
            }
            if ui.button("⏯").clicked() {
                self.display_pause_resume();
            }
            if ui.button("⏭").on_hover_text("Skip 5 seconds").clicked() {
                self.event_sender.send(Event::Skip5s).unwrap();
            }
            ui.add_space(10.);
            if ui.button("🗑").on_hover_text("clear list").clicked() {
                self.event_sender.send(Event::Stop).unwrap();
                self.config.current_pos = 0;
            }
        });
    }


}

fn display_time(time: &u64) -> String{
    let seconds = time % 60;
    let minutes = time / 60;
    format!("{}:{}", minutes, seconds)
}

impl Handler {
    fn display_dump_mode(&mut self, ui: &mut egui::Ui) {
        ui.checkbox(&mut self.config.dump_mode, "dump mode");
    }

    fn display_pause_resume(&mut self) {
        if self.config.is_playing {
            self.event_sender.send(Event::Pause).unwrap();
            self.config.is_playing = false;
        } else {
            self.event_sender.send(Event::Play).unwrap();
            self.config.is_playing = true;
        }
    }
}