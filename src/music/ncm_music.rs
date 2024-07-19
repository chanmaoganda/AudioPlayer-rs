use std::path::Path;

use crate::MusicMedia;

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

impl MusicMedia for NcmMusic {
    fn as_source(self) -> std::io::Cursor<Vec<u8>> {
        std::io::Cursor::new(self.audio_data)
    }
}