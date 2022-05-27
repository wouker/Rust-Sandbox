use rodio::OutputStream;

//here we handle music-infrastructure
pub fn get_music_handler() -> (OutputStream, rodio::Sink) {
    let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let music_sink = rodio::Sink::try_new(&stream_handle).unwrap();
    music_sink.set_volume(0.1);

    (stream, music_sink)
}