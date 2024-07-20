use std::path::PathBuf;

pub enum Event {
    Append(PathBuf),
    Play,
    Pause,
    Stop,
    Next(PathBuf),
}