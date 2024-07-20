mod music;
mod play_handler;
mod gui;
mod event;

use std::{sync::mpsc::sync_channel, thread};

use gui::Handler;
use music::*;
use play_handler::MusicPlayer;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let (sender, receiver) = sync_channel(1);
    

    thread::spawn(move || {
        let music_player = MusicPlayer::new(receiver);
        music_player.run();
    });
    
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("AudioPlayer-rs", 
        native_options, 
        Box::new(
            |cc: &eframe::CreationContext| Ok(Box::new(Handler::new(cc, sender)))
        )
    ).unwrap();

    log::info!("Exiting");

    Ok(())
}