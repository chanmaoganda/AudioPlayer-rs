use std::path::PathBuf;

pub struct NcmMusic {
    audio_data: Vec<u8>,
}

impl NcmMusic {
    pub fn from_path(path: &PathBuf) -> anyhow::Result<Self> {
        let music_dump_lib::NcmMusic {
            metadata: _,
            audio_data,
            music_type: _,
        } = music_dump_lib::NcmDecoder::new(path).decode()?;
        Ok(Self {audio_data})
    }
}