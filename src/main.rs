use babilado_types::Event;
use std::io::{self, BufReader, Write};
use std::net::TcpStream;
use std::thread;

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();

    let mut stream = TcpStream::connect("127.0.0.1:9999")?;

    thread::spawn({
        let stream = stream.try_clone()?;

        || {
            if let Err(e) = listen_for_events(BufReader::new(stream)) {
                eprintln!("Error: {:?}", e);
            }
        }
    });

    loop {
        let mut message = String::new();
        stdin.read_line(&mut message)?;

        jsonl::write(&mut stream, &Event::NewMessage { text: message })?;
        stream.flush()?;
    }
}

fn listen_for_events(mut stream: BufReader<TcpStream>) -> anyhow::Result<()> {
    loop {
        let event: Event = jsonl::read(&mut stream)?;
        dbg!(event);
    }
}
