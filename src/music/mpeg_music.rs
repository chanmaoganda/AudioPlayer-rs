use std::path::Path;

use crate::MusicMedia;


pub struct MpegMusic {
    audio_data: Vec<u8>,
}

impl MpegMusic {
    pub fn from_path(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let audio_data = std::fs::read(path)?;
        Ok(Self { audio_data })
    }
}

impl MusicMedia for MpegMusic {
    fn as_source(self) -> std::io::Cursor<Vec<u8>> {
        std::io::Cursor::new(self.audio_data)
    }
}