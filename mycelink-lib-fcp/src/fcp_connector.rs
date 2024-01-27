use std::error::Error;
use crate::model::message::Message;
use crate::model::unique_identifier::UniqueIdentifier;
use crate::peekable_reader::PeekableReader;
use std::sync::Weak;
use tokio::net::tcp::{ReadHalf, WriteHalf};
use tokio::net::TcpStream;

pub struct FCPConnector<'stream> {
    tx: WriteHalf<'stream>,
    rx: PeekableReader<ReadHalf<'stream>>,

    identity_listeners: Vec<IdentityListener>,
    default_listeners: Vec<Box<DecliningListener>>,
}

impl<'stream> FCPConnector<'stream> {
    pub fn new(stream: &'stream mut TcpStream) -> Self {
        let (rx, tx) = stream.split();

        let rx = PeekableReader::new(rx);

        Self {
            tx,
            rx,
            identity_listeners: Vec::new(),
            default_listeners: Vec::new(),
        }
    }

    pub async fn listen(mut self) {
        loop {
            match Message::decode(&mut self.rx).await {
                Ok(message) => {
                    self.handle_message(message).await;
                }
                Err(err) => {
                    todo!()
                }
            }
        }
    }

    async fn handle_message(&mut self, message: Message) -> Result<(), dyn Error> {
        match message.fields().get("Identity")
    }
}

struct IdentityListener {
    listen_to: UniqueIdentifier,
    action: Weak<dyn Fn(Message)>,
}

impl IdentityListener {
    pub fn is_active(&self) -> bool {
        self.action.strong_count() > 0
    }
}

struct DecliningListener {
    filter: dyn Fn(&Message) -> bool,
    action: dyn Fn(Message),
}
