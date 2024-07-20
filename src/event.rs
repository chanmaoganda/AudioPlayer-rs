use std::path::PathBuf;

pub enum Event {
    Quit,
    Play,
    Pause,
    Stop,
    Next(PathBuf),
}