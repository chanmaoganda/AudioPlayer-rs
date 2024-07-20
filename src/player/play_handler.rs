use std::{path::PathBuf, sync::mpsc::{Receiver, SyncSender}, thread, time::Duration};

use rodio::{OutputStream, Sink};

use crate::event::Event;

use super::audio::Audio;

pub struct MusicPlayer {
    sink: Sink,
    _stream: OutputStream,
    event_receiver: Receiver<Event>,
    pos_sender: SyncSender<Duration>,
}

impl MusicPlayer {
    pub fn new(event_receiver: Receiver<Event>, pos_sender: SyncSender<Duration>) -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        Self {
            sink,
            _stream,
            event_receiver,
            pos_sender,
        }
    }

    pub fn run(self) {
        loop {
            thread::sleep(Duration::from_millis(15));
            if !self.sink.empty() {
                self.pos_sender.send(self.sink.get_pos()).unwrap();
            }
            match self.event_receiver.try_recv() {
                Ok(event) => {
                    match event {
                        Event::Append(path) => self.append(path),
                        Event::Play => self.play(),
                        Event::Pause => self.pause(),
                        Event::Stop => self.stop(),
                        Event::Next(music) => self.next(music),
                    }
                }
                _ => {}
            }
        }
    }

    fn play(&self) {
        self.sink.play();
    }

    fn pause(&self) {
        self.sink.pause();
    }

    fn stop(&self) {
        self.sink.stop();
    }

    fn next(&self, path: PathBuf) {
        let audio = Audio::new(&path);
        self.sink.stop();
        self.sink.append(audio.into_decoder());
        self.sink.play();
    }

    fn append(&self, path: PathBuf) {
        let audio = Audio::new(&path);
        self.sink.append(audio.into_decoder());
    }
}