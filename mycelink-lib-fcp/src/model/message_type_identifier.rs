use crate::decode_error::DecodeError;
use crate::decode_error::DecodeError::UnexpectedEOF;
use crate::model::message_type_identifier::ClientMessageType::{ClientGet, ClientHello};
use crate::model::message_type_identifier::NodeMessageType::NodeHello;
use crate::peekable_reader::Peeker;
use std::ops::Deref;
use tokio::io::AsyncRead;

pub const CLIENT_MESSAGE_TYPES: &[ClientMessageType] = &[ClientHello, ClientGet];
pub const NODE_MESSAGE_TYPES: &[NodeMessageType] = &[NodeHello];

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ClientMessageType {
    ClientHello,
    ClientGet,
    ClientPut,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NodeMessageType {
    NodeHello,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MessageType {
    Client(ClientMessageType),
    Node(NodeMessageType),
}

impl NodeMessageType {
    /// Gets the str representing the MessageIdentifier (e.g. "ClientHello")
    pub fn name(&self) -> &'static str {
        match self {
            NodeHello => "NodeHello",
        }
    }
}

impl ClientMessageType {
    /// Gets the str representing the MessageIdentifier (e.g. "ClientHello")
    pub fn name(&self) -> &'static str {
        match self {
            ClientHello => "ClientHello",
            ClientGet => "ClientGet",
            ClientMessageType::ClientPut => "ClientPut",
        }
    }
}

impl MessageType {
    pub fn name(&self) -> &'static str {
        match self {
            MessageType::Client(inner) => inner.name(),
            MessageType::Node(inner) => inner.name(),
        }
    }

    pub fn is_specific_node_message(&self, matches: &NodeMessageType) -> bool {
        match self {
            MessageType::Client(_) => false,
            MessageType::Node(inner) => inner == matches,
        }
    }
}

impl TryFrom<&str> for ClientMessageType {
    type Error = DecodeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        CLIENT_MESSAGE_TYPES
            .iter()
            .find(|e| e.name() == value)
            .copied()
            .ok_or_else(|| DecodeError::UnknownMessageType { got: value.into() })
    }
}

impl TryFrom<&str> for NodeMessageType {
    type Error = DecodeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        NODE_MESSAGE_TYPES
            .iter()
            .find(|e| e.name() == value)
            .copied()
            .ok_or_else(|| DecodeError::UnknownMessageType { got: value.into() })
    }
}

impl TryFrom<&str> for MessageType {
    type Error = DecodeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        NodeMessageType::try_from(value)
            .map(MessageType::Node)
            .or(ClientMessageType::try_from(value).map(MessageType::Client))
    }
}

impl MessageType {
    pub async fn decode(
        peeker: &mut Peeker<'_, impl AsyncRead + Unpin>,
    ) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let peeked_identifier = peeker.next_contentful_line().await?.ok_or(UnexpectedEOF)?;

        let res = MessageType::try_from(peeked_identifier.deref())?;

        Ok(res)
    }
}
