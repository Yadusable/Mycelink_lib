use crate::fcp_connector::filters::MessageFilter;
use crate::messages::client_hello::{ClientHelloMessage, EXPECTED_VERSION};
use crate::model::message::Message;
use crate::peekable_reader::PeekableReader;
use log::error;
use std::convert::Infallible;
use std::sync::Mutex;
use tokio::io::AsyncWriteExt;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;

pub struct FCPConnector {
    tx: Mutex<OwnedWriteHalf>,
    rx: tokio::sync::Mutex<PeekableReader<OwnedReadHalf>>,

    listeners: tokio::sync::Mutex<Vec<Listener>>,
}

impl FCPConnector {
    pub async fn new(stream: TcpStream, client_name: &str) -> Result<Self, tokio::io::Error> {
        let (rx, tx) = stream.into_split();

        let rx = PeekableReader::new(rx);

        let s = Self {
            tx: Mutex::new(tx),
            rx: tokio::sync::Mutex::new(rx),
            listeners: tokio::sync::Mutex::new(Vec::new()),
        };

        log::info!("Connecting to Freenet over FCP");
        let client_hello = ClientHelloMessage {
            name: client_name.into(),
            version: EXPECTED_VERSION,
        };

        s.send(client_hello).await?;

        Ok(s)
    }

    pub async fn listen(&self) {
        let mut rx = self
            .rx
            .try_lock()
            .expect("FCPConnector::listen my only be called once per struct");
        loop {
            match Message::decode(&mut rx).await {
                Ok(message) => match self.handle_message(message).await {
                    Ok(_) => {}
                    Err(err) => {
                        error!("Received error while handling message {err}")
                    }
                },
                Err(err) => {
                    error!("Error while decoding message {err}");
                    todo!()
                }
            }
        }
    }

    async fn handle_message(&self, message: Message) -> Result<(), Infallible> {
        log::debug!("Received message {message:?}");
        let mut has_marked_for_delete = false;
        let mut listeners = self.listeners.lock().await;
        let listener = listeners
            .iter_mut()
            .map(|e| {
                has_marked_for_delete |= e.is_marked_for_delete();
                e
            }) // Detect old listeners
            .filter(|e| !e.is_marked_for_delete())
            .find(|e| e.filter(&message));

        match listener {
            Some(listener) => listener.action(message).await,
            None => {
                log::warn!("Received Message with no listener for it {message:?}")
            }
        }

        if has_marked_for_delete {
            listeners.retain(|e| !e.is_marked_for_delete())
        }

        Ok(())
    }

    pub async fn add_listener(&self, listener: Listener) {
        let mut cursor = 0;
        let mut listeners = self.listeners.lock().await;
        while cursor < listeners.len() && listeners[cursor].priority() < listener.priority() {
            cursor += 1;
        }
        listeners.insert(cursor, listener);
    }

    pub async fn send(&self, message: impl Into<Message>) -> Result<(), tokio::io::Error> {
        let message = message.into();
        let bytes = message.encode();
        let mut tx = self.tx.lock().unwrap();
        tx.write_all(bytes.as_slice()).await
    }
}

pub struct Listener {
    filters: Vec<Box<MessageFilter>>,
    priority: i8,
    notify: tokio::sync::mpsc::Sender<Message>,
    marked_for_deletion: bool,
}

impl Listener {
    pub const DEFAULT_PRIORITY: i8 = 0;

    pub fn new(
        filters: Vec<Box<MessageFilter>>,
        priority: i8,
        notify: tokio::sync::mpsc::Sender<Message>,
    ) -> Self {
        Self {
            filters,
            priority,
            notify,
            marked_for_deletion: false,
        }
    }

    pub fn priority(&self) -> i8 {
        self.priority
    }

    pub fn filter(&self, message: &Message) -> bool {
        !self.marked_for_deletion
            && self.filters.iter().filter(|e| (*e)(message)).count() == self.filters.len()
    }

    pub async fn action(&mut self, message: Message) {
        let _ = self.notify.send(message).await;
    }

    pub fn is_marked_for_delete(&self) -> bool {
        self.notify.is_closed()
    }
}

pub mod filters {
    use crate::model::message::Message;
    use crate::model::message_type_identifier::{MessageType, NodeMessageType};
    use crate::model::unique_identifier::UniqueIdentifier;

    pub type MessageFilter = dyn (Fn(&Message) -> bool) + Send;

    pub fn identity_filter(identity: UniqueIdentifier) -> Box<MessageFilter> {
        Box::new(move |message| {
            message
                .fields()
                .get("Identifier")
                .and_then(|identity_field| identity_field.value().try_into().ok())
                .map(|identifier: UniqueIdentifier| identifier == identity)
                .unwrap_or(false)
        })
    }

    pub fn type_filter(aim_message_type: NodeMessageType) -> Box<MessageFilter> {
        Box::new(move |message| {
            if let MessageType::Node(message_type) = message.message_type() {
                message_type == aim_message_type
            } else {
                false
            }
        })
    }
}
