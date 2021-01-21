use babilado_types::{Event, Message, Nickname, Tag, User, UserId};
use std::io::{self, BufReader, Write};
use std::net::TcpStream;
use std::thread;

fn main() -> anyhow::Result<()> {
    let stdin = io::stdin();
    let nickname = choose_nickname(&stdin)?;
    let tag = choose_tag(&stdin)?;
    let mut random_seed = [0; 16];
    getrandom::getrandom(&mut random_seed)?;
    let user_id = UserId::gen(&mut oorandom::Rand64::new(u128::from_le_bytes(random_seed)));
    let mut stream = TcpStream::connect("127.0.0.1:9999")?;

    jsonl::write(
        &mut stream,
        &User {
            nickname,
            tag,
            id: user_id,
        },
    )?;

    thread::spawn({
        let stream = stream.try_clone()?;

        || {
            if let Err(e) = listen_for_events(BufReader::new(stream)) {
                eprintln!("Error: {:?}", e);
            }
        }
    });

    loop {
        let message = Message {
            body: {
                let mut body = String::new();
                stdin.read_line(&mut body)?;

                body
            },
            author: user_id,
        };

        jsonl::write(&mut stream, &Event::NewMessage(message))?;
        stream.flush()?;
    }
}

fn listen_for_events(mut stream: BufReader<TcpStream>) -> anyhow::Result<()> {
    loop {
        let event: Event = jsonl::read(&mut stream)?;
        dbg!(event);
    }
}

fn choose_nickname(stdin: &io::Stdin) -> anyhow::Result<Nickname> {
    loop {
        let nickname = prompt_and_read(stdin, "Choose a nickname")?;
        if !nickname
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
        {
            println!("Nicknames can only contain a-z, 0-9, hyphens and underscores.");
        } else {
            return Ok(Nickname::new(nickname));
        }
    }
}

fn choose_tag(stdin: &io::Stdin) -> anyhow::Result<Tag> {
    loop {
        let tag = prompt_and_read(
            stdin,
            "Pick a four digit tag to accompany your nickname e.g. user#1234",
        )?;
        if tag.len() != 4 {
            println!("Tag length must be 4 digits!");
        } else if !tag.chars().all(|c| c.is_ascii_digit()) {
            println!("Tag may only include numbers!");
        } else {
            return Ok(Tag::new(tag.parse().unwrap()));
        }
    }
}

fn prompt_and_read(stdin: &io::Stdin, prompt: &str) -> anyhow::Result<String> {
    print!("{}: ", prompt);
    io::stdout().flush()?;
    let mut response = String::new();
    stdin.read_line(&mut response)?;
    Ok(response[0..response.len() - 1].to_string())
}
