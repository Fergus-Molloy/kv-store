mod message;
mod store;
mod tcp;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;

use message::{Message, Reply};

fn main() -> std::io::Result<()> {
    let store = crate::store::start();
    let listener = TcpListener::bind("localhost:8000")?;
    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                let store = store.clone();
                std::thread::spawn(move || handle_client(s, &store));
            }
            Err(e) => return Err(e),
        }
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream, store: &mpsc::Sender<Message>) {
    loop {
        match tcp::recieve_message(&mut stream) {
            Ok(message) => {
                println!("Received message: {message}");
                let Ok(parsed) = Message::parse(&message) else {
                    eprintln!("Could not parse message");
                    let Ok(()) = tcp::write_msg(&mut stream, "R|500") else {
                        eprintln!("Could not send message to client");
                        return;
                    };
                    continue;
                };
                match parsed {
                    (msg, None) => handle_put(&mut stream, store, msg),
                    (msg, Some(rx)) => handle_get(&mut stream, store, &rx, msg),
                }
            }
            Err(err) => {
                eprintln!("Error handling client: {err}");
                return;
            }
        }
    }
}

fn handle_put(stream: &mut TcpStream, store: &mpsc::Sender<Message>, msg: Message) {
    let mut r = "PR|201";
    let resp = store.send(msg);
    if resp.is_err() {
        eprintln!("Could not send message to store");
        r = "PR|500";
    }
    let Ok(()) = tcp::write_msg(stream, r) else {
        eprintln!("Could not send message to client");
        return;
    };
}

fn handle_get(
    stream: &mut TcpStream,
    store: &mpsc::Sender<Message>,
    rx: &mpsc::Receiver<Reply>,
    msg: Message,
) {
    let resp = store.send(msg);
    let r = {
        if resp.is_err() {
            eprintln!("Could not send message to store");
            String::from("GR|500")
        } else {
            match rx.recv() {
                Ok(Some(value)) => {
                    println!("Got value: {value}");
                    format!("GR|200|{value}")
                }
                Ok(None) => {
                    eprintln!("Key not in store");
                    String::from("GR|404")
                }
                _ => {
                    eprintln!("Could not send message to store");
                    String::from("GR|500")
                }
            }
        }
    };
    println!("Sending message {r}");
    let Ok(()) = tcp::write_msg(stream, &r) else {
        eprintln!("Could not send message to client");
        return;
    };
}
