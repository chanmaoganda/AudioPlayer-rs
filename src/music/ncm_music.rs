use std::{io::Cursor, path::Path};

use crate::Decodable;

#[derive(Clone)]
pub struct NcmMusic {
    audio_data: Vec<u8>,
}

impl NcmMusic {
    pub fn from_path(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let music_dump_lib::NcmMusic {
            metadata: _,
            audio_data,
            music_type: _,
        } = music_dump_lib::NcmDecoder::new(path).decode()?;
        Ok(Self {audio_data})
    }
}

impl Decodable for NcmMusic {
    fn get_cursor(&self) -> Cursor<Vec<u8>> {
        Cursor::new(self.audio_data.clone())
    }
}