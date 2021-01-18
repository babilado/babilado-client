use babilado_types::Event;
use jsonl::Connection;
use std::io::{self, BufReader};
use std::net::TcpStream;

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();

    let stream = TcpStream::connect("127.0.0.1:9999")?;
    let mut server_connection = Connection::new(BufReader::new(stream.try_clone()?), stream);

    loop {
        let mut message = String::new();
        stdin.read_line(&mut message)?;

        server_connection.write(&Event::NewMessage { text: message })?;
        server_connection.flush()?;

        let response: Event = server_connection.read()?;
        dbg!(response);
    }
}
