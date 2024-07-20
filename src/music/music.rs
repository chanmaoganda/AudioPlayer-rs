use std::{io::Cursor, path::{Path, PathBuf}};

use lofty::file::{AudioFile, FileType, TaggedFileExt};

use crate::{Decodable, FlacMusic, MpegMusic, NcmMusic, WavMusic};

pub struct Music {
    pub path: PathBuf,
    pub name: String,
    pub audio_info: Option<AudioInfo>,
}

impl Music {
    pub fn new(path: impl AsRef<Path>) -> Self {
        let name = path.as_ref().file_name().unwrap()
            .to_str().unwrap().to_string();
        Self {
            path: PathBuf::from(path.as_ref()),
            name,
            audio_info: None,
        }
    }

    pub fn parse_audio_info(&mut self) {
        let audio_info = AudioInfo::from_path(&self.path);
        self.audio_info.replace(audio_info);
    }
}

pub struct AudioInfo {
    pub duration: u64,
    pub decodable: Box<dyn Decodable>,
}

impl AudioInfo {
    pub fn from_path(path: impl AsRef<Path>) -> Self {
        let file = lofty::read_from_path(&path).unwrap();
        let properties = file.properties();
        let duration = properties.duration();
        let decodable: Box<dyn Decodable> = match file.file_type() {
            FileType::Custom("ncm") => Box::new(NcmMusic::from_path(path).unwrap()),
            FileType::Mpeg => Box::new(MpegMusic::from_path(path).unwrap()),
            FileType::Wav => Box::new(WavMusic::from_path(path).unwrap()),
            FileType::Flac => Box::new(FlacMusic::from_path(path).unwrap()),
            _ => panic!("Unsupported file format"),
        };
        Self {
            duration: duration.as_secs(),
            decodable,
        }
    }

    pub fn decode_audio(&self) -> Cursor<Vec<u8>> {
        self.decodable.get_cursor()
    }
}
