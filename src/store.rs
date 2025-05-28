use std::{
    collections::HashMap,
    sync::mpsc::{self, Receiver},
};

use crate::message::Message;

pub fn start() -> mpsc::Sender<Message> {
    let (tx, rx) = mpsc::channel();

    std::thread::spawn(move || store_listen(&rx));
    tx
}

fn store_listen(rx: &Receiver<Message>) {
    let mut store: HashMap<String, String> = HashMap::new();
    for msg in rx {
        match msg {
            Message::Put { key, value } => {
                store.insert(key, value);
            }
            Message::Get { key, reply } => {
                let value = store.get(&key).cloned();
                let Ok(()) = reply.send(value) else {
                    continue;
                };
            }
        }
    }
}
