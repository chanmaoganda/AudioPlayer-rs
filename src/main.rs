mod music;

pub use music::*;
use rodio::{Decoder, OutputStream, Sink};

fn main() -> anyhow::Result<()> {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let _mpeg_music = MpegMusic::from_path("./target/audios/1.mp3")?;
    let ncm_music = NcmMusic::from_path("./target/VipSongsDownload/8bite - honest.ncm")?;
    let source = Decoder::new(ncm_music.as_source())?;
    sink.append(source);

    sink.sleep_until_end();
    Ok(())
}