mod mpeg_music;
mod ncm_music;
mod music;

use std::io::Cursor;

pub use mpeg_music::MpegMusic;
pub use ncm_music::NcmMusic;
pub use music::Music;

/// This trait guarantees the implementation struct is decoded when the instance is created.
pub trait Decodable {
    fn get_cursor(&self) -> Cursor<Vec<u8>>;
}