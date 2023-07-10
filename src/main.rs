use std::error::Error;
use stream_download::StreamDownload;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    let reader =
        StreamDownload::new_http("https://stream.gensokyoradio.net/3".parse().unwrap()).unwrap();

    sink.append(rodio::Decoder::new(reader).unwrap());

    sink.sleep_until_end();

    Ok(())
}
