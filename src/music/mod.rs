mod mpeg_music;
mod ncm_music;

pub trait MusicMedia {
    fn as_source(&self);
}

