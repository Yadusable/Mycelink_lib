use crate::model::message::Message;
use std::sync::{Mutex, MutexGuard, PoisonError};

use crate::fcp_connector::filters::MessageFilter;
use crate::peekable_reader::PeekableReader;
use tokio::io::AsyncWriteExt;
use tokio::net::tcp::{ReadHalf, WriteHalf};
use tokio::net::TcpStream;

pub struct FCPConnector<'stream> {
    tx: Mutex<WriteHalf<'stream>>,
    rx: Mutex<PeekableReader<ReadHalf<'stream>>>,

    listeners: Mutex<Vec<Listener>>,
}

impl<'stream> FCPConnector<'stream> {
    pub fn new_and_listen(stream: &'stream mut TcpStream) -> Self {
        let (rx, tx) = stream.split();

        let rx = PeekableReader::new(rx);

        Self {
            tx: Mutex::new(tx),
            rx: Mutex::new(rx),
            listeners: Mutex::new(Vec::new()),
        }
    }

    async fn listen(&self) {
        let mut rx = self
            .rx
            .try_lock()
            .expect("FCPConnector::listen my only be called once per struct");
        loop {
            match Message::decode(&mut rx).await {
                Ok(message) => {
                    self.handle_message(message).await;
                }
                Err(_err) => {
                    todo!()
                }
            }
        }
    }

    async fn handle_message(
        &self,
        message: Message,
    ) -> Result<(), PoisonError<MutexGuard<'_, Vec<Listener>>>> {
        let mut has_marked_for_delete = false;
        let mut listeners = self.listeners.lock()?;
        let listener = listeners
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
            listeners.retain(|e| !e.is_marked_for_delete())
        }

        Ok(())
    }

    pub fn add_listener(&mut self, listener: Listener) {
        let mut cursor = 0;
        let mut listeners = self.listeners.lock().unwrap();
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

pub type Action = dyn Fn(Message) -> bool;
pub struct Listener {
    filters: Vec<Box<MessageFilter>>,
    priority: i8,
    action: Box<Action>,
    marked_for_deletion: bool,
}

impl Listener {
    pub const DEFAULT_PRIORITY: i8 = 0;

    pub fn new(
        filters: Vec<Box<MessageFilter>>,
        priority: i8,
        action: impl Into<Box<Action>>,
    ) -> Self {
        Self {
            filters,
            priority,
            action: action.into(),
            marked_for_deletion: false,
        }
    }

    pub fn priority(&self) -> i8 {
        self.priority
    }

    pub fn filter(&self, message: &Message) -> bool {
        self.marked_for_deletion
            && self.filters.iter().filter(|e| (*e)(message)).count() == self.filters.len()
    }

    pub fn action(&mut self, message: Message) {
        self.marked_for_deletion |= (*self.action)(message)
    }

    pub fn is_marked_for_delete(&self) -> bool {
        self.marked_for_deletion
    }
}

pub mod filters {
    use crate::model::message::Message;
    use crate::model::message_type_identifier::{MessageType, NodeMessageType};
    use crate::model::unique_identifier::UniqueIdentifier;

    pub type MessageFilter = dyn Fn(&Message) -> bool;

    pub fn identity_filter(identity: UniqueIdentifier) -> Box<MessageFilter> {
        Box::new(move |message| {
            message
                .fields()
                .get("Identity")
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
