use std::{path::PathBuf, sync::Arc};

use egui::mutex::RwLock;

pub struct DialogHandler {
    file_list: Arc<RwLock<Vec<PathBuf>>>,
    directory: Arc<RwLock<Option<String>>>,
    support_formats: Vec<String>,
    /// update list for every 10 ticks
    cycle_tick: u8,
}

impl DialogHandler {
    pub fn new(directory: Arc<RwLock<Option<String>>>) -> Self {
        Self {
            file_list: Arc::new(RwLock::new(Vec::new())),
            directory,
            support_formats: vec![
                "mp3".to_string(), 
                "ncm".to_string(),
                "flac".to_string(),
                "wav".to_string(),
                ],
            cycle_tick: 0,
        }
    }

    pub fn list_ref(&self) -> Arc<RwLock<Vec<PathBuf>>> {
        self.file_list.clone()
    }

    pub fn update_list(&mut self) {
        if !self.check_update() {
            return;
        }
        let dir_ref = self.directory.as_ref();
        if dir_ref.read().is_none() {
            return;
        }
        let mut file_list_task = self.file_list.write();
        file_list_task.clear();
        self.support_formats.iter().for_each(|format| {
            for entry in glob::glob(
                &format!("{}/*.{}", dir_ref.read().as_ref().unwrap(), format)).unwrap() {
                let entry_path = entry.unwrap();
                file_list_task.push(entry_path);
            }
        });
    }
}

impl DialogHandler {
    fn check_update(&mut self) -> bool {
        self.cycle_tick += 1;
        if self.cycle_tick == 200  {
            self.cycle_tick = 0;
            return true;
        }
        if self.cycle_tick % 10 != 0 { true } 
        else { false }
    }
}