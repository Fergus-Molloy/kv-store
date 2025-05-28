use std::sync::mpsc;

#[derive(Clone)]
pub enum Message {
    Put {
        key: String,
        value: String,
    },
    Get {
        key: String,
        reply: mpsc::Sender<Reply>,
    },
}

pub type Reply = Option<String>;

impl Message {
    pub fn parse(p: &str) -> Result<(Self, Option<mpsc::Receiver<Reply>>), &'static str> {
        let parts: Vec<&str> = p.split('|').collect();
        match parts[..] {
            ["P", key, value] => Ok((
                Self::Put {
                    key: key.into(),
                    value: value.into(),
                },
                None,
            )),
            ["G", key] => {
                let (tx, rx) = mpsc::channel();
                Ok((
                    Self::Get {
                        key: key.into(),
                        reply: tx,
                    },
                    Some(rx),
                ))
            }
            _ => Err("Could not parse message"),
        }
    }
}
