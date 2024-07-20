use std::{path::PathBuf, sync::mpsc::Receiver, thread, time::Duration};

use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};

use crate::{event::Event, Music};

pub struct MusicPlayer {
    sink: Sink,
    stream_handle: OutputStreamHandle,
    stream: OutputStream,
    event_receiver: Receiver<Event>,
}

impl MusicPlayer {
    pub fn new(receiver: Receiver<Event>) -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        Self {
            sink,
            stream_handle,
            stream: _stream,
            event_receiver: receiver,
        }
    }

    pub fn run(self) {
        loop {
            thread::sleep(Duration::from_millis(15));
            match self.event_receiver.try_recv() {
                Ok(event) => {
                    match event {
                        Event::Quit => {
                            self.quit();
                            break;
                        },
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

    fn quit(&self) {
        self.sink.stop();
    }

    fn next(&self, path: PathBuf) {
        log::info!("received next music");
        self.sink.stop();
        let mut music = Music::new(path);
        music.parse_audio_info();
        let audio = music.audio_info.unwrap().decode_audio();
        self.sink.append(Decoder::new(audio).unwrap());
        self.sink.play();
        log::info!("next music playing");
    }
}