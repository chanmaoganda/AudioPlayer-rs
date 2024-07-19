mod mpeg_music;
mod ncm_music;

use std::io::Cursor;

pub use mpeg_music::MpegMusic;
pub use ncm_music::NcmMusic;

pub trait Decodable {
    fn into_cursor(self) -> Cursor<Vec<u8>>;
}

pub struct AudioItem {
    pub name: String,
    pub cursor: Cursor<Vec<u8>>,
    pub duration: u64,
    pub tag: id3::Tag,
}

