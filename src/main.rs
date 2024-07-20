mod player;
mod gui;
mod event;

use std::{sync::mpsc::sync_channel, thread};

use gui::Handler;
use player::MusicPlayer;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let (event_sender, event_receiver) = sync_channel(1);
    let (pos_sender, pos_receiver) = sync_channel(1);

    thread::spawn(move || {
        let music_player = MusicPlayer::new(event_receiver, pos_sender);
        music_player.run();
    });
    
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("AudioPlayer-rs", 
        native_options, 
        Box::new(
            |cc: &eframe::CreationContext| Ok(Box::new(Handler::new(cc, event_sender, pos_receiver)))
        )
    ).unwrap();

    log::info!("Exiting");

    Ok(())
}