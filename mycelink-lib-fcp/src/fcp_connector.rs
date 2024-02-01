use crate::model::message::Message;
use crate::model::message_type_identifier::MessageType;
use crate::model::unique_identifier::UniqueIdentifier;
use crate::peekable_reader::PeekableReader;
use tokio::io::AsyncWriteExt;
use tokio::net::tcp::{ReadHalf, WriteHalf};
use tokio::net::TcpStream;

pub struct FCPConnector<'stream> {
    tx: WriteHalf<'stream>,
    rx: PeekableReader<ReadHalf<'stream>>,

    listeners: Vec<Box<dyn Listener>>,
}

impl<'stream> FCPConnector<'stream> {
    pub fn new(stream: &'stream mut TcpStream) -> Self {
        let (rx, tx) = stream.split();

        let rx = PeekableReader::new(rx);

        Self {
            tx,
            rx,
            listeners: Vec::new(),
        }
    }

    pub async fn listen(mut self) {
        loop {
            match Message::decode(&mut self.rx).await {
                Ok(message) => {
                    self.handle_message(message).await;
                }
                Err(_err) => {
                    todo!()
                }
            }
        }
    }

    async fn handle_message(&mut self, message: Message) {
        let mut has_marked_for_delete = false;
        let listener = self
            .listeners
            .iter_mut()
            .map(|e| {
                has_marked_for_delete |= e.is_marked_for_delete();
                e
            }) // Detect old listeners
            .filter(|e| !e.is_marked_for_delete())
            .find(|e| e.filter(&message));
        match listener {
            Some(listener) => listener.action(message),
            None => {
                log::warn!("Received Message with no listener for it {message:?}")
            }
        }

        if has_marked_for_delete {
            self.listeners.retain(|e| !e.is_marked_for_delete())
        }
    }

    pub fn add_listener(&mut self, listener: Box<dyn Listener>) {
        let mut cursor = 0;
        while cursor < self.listeners.len()
            && self.listeners[cursor].priority() < listener.priority()
        {
            cursor += 1;
        }
        self.listeners.insert(cursor, listener);
    }

    pub async fn send(&mut self, message: impl Into<Message>) -> Result<(), tokio::io::Error> {
        let message = message.into();
        let bytes = message.encode();
        self.tx.write_all(bytes.as_slice()).await
    }
}

pub trait Listener {
    fn priority(&self) -> i8;
    fn filter(&self, message: &Message) -> bool;
    fn action(&mut self, message: Message);
    fn is_marked_for_delete(&self) -> bool;
}

struct IdentityListener {
    listen_to: UniqueIdentifier,
    on_message: Box<dyn Fn(Message) -> bool>,
    is_marked_for_delete: bool,
}

impl Listener for IdentityListener {
    fn priority(&self) -> i8 {
        10
    }

    fn filter(&self, message: &Message) -> bool {
        message
            .fields()
            .get("Identity")
            .and_then(|identity_field| identity_field.value().try_into().ok())
            .map(|identifier: UniqueIdentifier| identifier == self.listen_to)
            .unwrap_or(false)
    }

    fn action(&mut self, message: Message) {
        self.is_marked_for_delete |= (*self.on_message)(message)
    }

    fn is_marked_for_delete(&self) -> bool {
        self.is_marked_for_delete
    }
}

struct TypeListener {
    listen_to: MessageType,
    on_message: Box<dyn Fn(Message) -> bool>,
    is_marked_for_delete: bool,
}

impl Listener for TypeListener {
    fn priority(&self) -> i8 {
        5
    }

    fn filter(&self, message: &Message) -> bool {
        message.message_type() == self.listen_to
    }

    fn action(&mut self, message: Message) {
        self.is_marked_for_delete |= (*self.on_message)(message)
    }

    fn is_marked_for_delete(&self) -> bool {
        self.is_marked_for_delete
    }
}
