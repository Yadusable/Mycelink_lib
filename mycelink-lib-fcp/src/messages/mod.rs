pub mod client_get;
pub mod client_hello;
pub mod node_hello;

use crate::messages::client_hello::ClientHelloMessage;
use crate::messages::node_hello::NodeHelloMessage;
use crate::model::message::Message;

pub enum ClientMessage {
    ClientHello(ClientHelloMessage),
}

impl From<ClientMessage> for Message {
    fn from(value: ClientMessage) -> Self {
        match value {
            ClientMessage::ClientHello(inner) => inner.into(),
        }
    }
}

pub enum NodeMessage {
    NodeHello(NodeHelloMessage),
}

pub struct MessagePayload {}
