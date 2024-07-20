use std::{io::Cursor, path::PathBuf};

use rodio::Decoder;

pub struct Audio {
    pub data: Cursor<Vec<u8>>,
}

impl Audio {
    pub fn new(path: &PathBuf) -> Self {
        Self { data: parser::parser_audio(path) }
    }

    pub fn into_decoder(self) -> Decoder<Cursor<Vec<u8>>> {
        Decoder::new(self.data).unwrap()
    }
}

pub mod parser {
    use std::{io::Cursor, path::PathBuf, time::Duration};

    use rodio::{Decoder, Source};

    use super::audio_util;

    pub fn parser_audio(path: &PathBuf) -> Cursor<Vec<u8>> {
        let extension = path.extension().unwrap();
        match extension.to_str().unwrap() {
            "ncm" => Cursor::new(audio_util::dump_ncm(path)),
            "mp3" | "flac" | "wav" => Cursor::new(audio_util::dump_simple_audio(path)),
            _ => panic!("Unsupported audio format"),
        }
    }

    pub fn get_duration(path: &PathBuf) -> Option<Duration> {
        Decoder::new(parser_audio(path)).unwrap().total_duration()
    }
}

mod audio_util {
    use std::path::Path;

    pub fn dump_ncm(path: impl AsRef<Path>) -> Vec<u8> {
        let mut decoder = music_dump_lib::NcmDecoder::new(path);
        let music_dump_lib::NcmMusic { 
            metadata: _, 
            audio_data, 
            music_type: _ } = decoder.decode().unwrap();
        audio_data
    }

    pub fn dump_simple_audio(path: impl AsRef<Path>) -> Vec<u8> {
        std::fs::read(path).unwrap()
    }

}