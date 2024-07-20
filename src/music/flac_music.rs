use std::{io::Cursor, path::Path};

use crate::Decodable;


pub struct FlacMusic {
    audio_data: Vec<u8>,
}

impl FlacMusic {
    pub fn from_path(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let audio_data = std::fs::read(path)?;
        Ok(Self { audio_data })
    }
}

impl Decodable for FlacMusic {
    fn get_cursor(&self) -> Cursor<Vec<u8>> {
        Cursor::new(self.audio_data.clone())
    }
}