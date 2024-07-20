use std::sync::Arc;

use egui::mutex::RwLock;

use crate::Music;

pub struct DialogHandler {
    file_list: Arc<RwLock<Vec<Music>>>,
    directory: Arc<RwLock<Option<String>>>,

    /// update list for every 10 ticks
    cycle_tick: u8,
}

impl DialogHandler {
    pub fn new(directory: Arc<RwLock<Option<String>>>) -> Self {
        Self {
            file_list: Arc::new(RwLock::new(Vec::new())),
            directory,
            cycle_tick: 0,
        }
    }

    pub fn list_ref(&self) -> Arc<RwLock<Vec<Music>>> {
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
        for entry in glob::glob(
            &format!("{}/*.{}", dir_ref.read().as_ref().unwrap(), "mp3")).unwrap() {
            let entry_path = entry.unwrap();
            file_list_task.push(Music::new(entry_path));
        }
        for entry in glob::glob(
            &format!("{}/*.{}", dir_ref.read().as_ref().unwrap(), "ncm")).unwrap() {
            let entry_path = entry.unwrap();
            file_list_task.push(Music::new(entry_path));
        }
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