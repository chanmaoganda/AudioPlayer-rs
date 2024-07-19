use std::path::PathBuf;


pub struct MpegMusic {
    audio_data: Vec<u8>,
}

impl MpegMusic {
    pub fn from_path(path: &PathBuf) -> anyhow::Result<Self> {
        let audio_data = std::fs::read(path)?;
        Ok(Self { audio_data })
    }
}