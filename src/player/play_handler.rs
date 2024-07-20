use std::{path::PathBuf, sync::mpsc::{Receiver, SyncSender}, thread, time::Duration};

use rodio::{OutputStream, Sink};

use crate::event::Event;

use super::audio::Audio;

pub struct MusicPlayer {
    sink: Sink,
    _stream: OutputStream,
    event_receiver: Receiver<Event>,
    pos_sender: SyncSender<Duration>,
    is_playing: bool,
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
            is_playing: false,
        }
    }

    pub fn run(mut self) {
        loop {
            thread::sleep(Duration::from_millis(5));
            if self.is_playing {
                self.pos_sender.send(self.sink.get_pos()).unwrap();
            }
            match self.event_receiver.try_recv() {
                Ok(event) => {
                    match event {
                        Event::Append(path) => self.append(path),
                        Event::Skip5s => self.skip_5s(),
                        Event::Rewind5s => self.rewind_5s(),
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

    fn stop(&mut self) {
        self.is_playing = false;
        self.sink.stop();
    }

    fn next(&mut self, path: PathBuf) {
        self.is_playing = true;
        let audio = Audio::new(&path);
        self.sink.stop();
        self.sink.append(audio.into_decoder());
        self.sink.play();
    }

    fn append(&mut self, path: PathBuf) {
        self.is_playing = true;
        let audio = Audio::new(&path);
        self.sink.append(audio.into_decoder());
    }

    fn skip_5s(&self) {
        let pos = self.sink.get_pos() + Duration::from_secs(5);
        self.sink.try_seek(pos).unwrap();
    }

    fn rewind_5s(&self) {
        let pos = self.sink.get_pos() - Duration::from_secs(5);
        self.sink.try_seek(pos).unwrap();
    }
}