mod wav_music;
mod flac_music;
mod mpeg_music;
mod ncm_music;
mod music;

use std::io::Cursor;

pub use wav_music::WavMusic;
pub use flac_music::FlacMusic;
pub use mpeg_music::MpegMusic;
pub use ncm_music::NcmMusic;
pub use music::Music;

/// This trait guarantees the implementation struct is decoded when the instance is created.
pub trait Decodable {
    fn get_cursor(&self) -> Cursor<Vec<u8>>;
}