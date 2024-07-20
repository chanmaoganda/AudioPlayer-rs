use std::path::PathBuf;

pub enum Event {
    Append(PathBuf),
    Play,
    Pause,
    Skip5s,
    Rewind5s,
    Stop,
    Next(PathBuf),
}