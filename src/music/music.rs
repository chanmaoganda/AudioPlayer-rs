use std::{io::Cursor, path::{Path, PathBuf}};

use crate::{Decodable, MpegMusic, NcmMusic};

pub struct Music {
    pub path: PathBuf,
    pub audio_info: Option<AudioInfo>,
}

impl Music {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: PathBuf::from(path.as_ref()),
            audio_info: None,
        }
    }

    pub fn parse_audio_info(&mut self) {
        let audio_info = AudioInfo::from_path(&self.path);
        self.audio_info.replace(audio_info);
    }
}

pub struct AudioInfo {
    pub name: String,
    pub duration: u64,
    pub tag: id3::Tag,
    pub decodable: Box<dyn Decodable>,
}

impl AudioInfo {
    pub fn from_path(path: impl AsRef<Path>) -> Self {
        let tag = id3::Tag::read_from_path(&path).unwrap();
        let name = path.as_ref().file_name().unwrap().to_str().unwrap().to_string();
        let duration = mp3_duration::from_path(&path).unwrap();
        let extension = path.as_ref().extension().unwrap();
        let decodable: Box<dyn Decodable> = if extension == "mp3" {
            Box::new(MpegMusic::from_path(path).unwrap())
        } else if extension == "ncm" {
            Box::new(NcmMusic::from_path(path).unwrap())
        } else {
            panic!("Unsupported file format: {:?}", extension);
        };
        Self {
            name,
            duration: duration.as_secs(),
            tag,
            decodable,
        }
    }

    pub fn decode_audio(&self) -> Cursor<Vec<u8>> {
        self.decodable.get_cursor()
    }
}

