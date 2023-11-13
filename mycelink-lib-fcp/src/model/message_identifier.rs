use crate::model::message_identifier::ClientMessageIdentifier::{ClientGet, ClientHello};
use crate::model::message_identifier::NodeMessageIdentifier::NodeHello;

pub const CLIENT_MESSAGE_IDENTIFIERS: &[ClientMessageIdentifier] = &[ClientHello, ClientGet];
pub const NODE_MESSAGE_IDENTIFIERS: &[NodeMessageIdentifier] = &[NodeHello];

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ClientMessageIdentifier {
    ClientHello,
    ClientGet,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NodeMessageIdentifier {
    NodeHello,
}

pub trait MessageIdentifier {
    fn name(&self) -> &'static str;
}

impl MessageIdentifier for NodeMessageIdentifier {
    /// Gets the str representing the MessageIdentifier (e.g. "ClientHello")
    fn name(&self) -> &'static str {
        match self {
            NodeHello => "NodeHello\n",
        }
    }
}

impl MessageIdentifier for ClientMessageIdentifier {
    /// Gets the str representing the MessageIdentifier (e.g. "ClientHello")
    fn name(&self) -> &'static str {
        match self {
            ClientHello => "ClientHello\n",
            ClientGet => "ClientGet\n",
        }
    }
}
