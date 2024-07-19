mod music;
mod play_handler;
mod gui;

use gui::Handler;
pub use music::*;
pub use play_handler::*;

fn main() -> anyhow::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("AudioPlayer-rs", 
        native_options, 
        Box::new(
            |_: &eframe::CreationContext| Ok(Box::new(Handler::new()))
        )
    ).unwrap();

    Ok(())
}