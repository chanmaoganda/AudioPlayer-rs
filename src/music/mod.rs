mod mpeg_music;
mod ncm_music;

use std::io::Cursor;

pub use mpeg_music::MpegMusic;
pub use ncm_music::NcmMusic;

pub trait MusicMedia {
    fn as_source(&self) -> Cursor<Vec<u8>>;
}