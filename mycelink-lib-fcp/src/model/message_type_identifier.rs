use crate::decode_error::DecodeError;
use crate::decode_error::DecodeError::UnexpectedEOF;
use crate::model::message_type_identifier::ClientMessageType::{ClientGet, ClientHello};
use crate::model::message_type_identifier::NodeMessageType::NodeHello;
use crate::peekable_reader::{PeekableReader, Peeker};
use crate::peekable_reader_legacy::PeekableReaderLegacy;
use std::ops::Deref;
use std::str::from_utf8;
use tokio::io::{AsyncRead, BufReader};

pub const CLIENT_MESSAGE_TYPES: &[ClientMessageType] = &[ClientHello, ClientGet];
pub const NODE_MESSAGE_TYPES: &[NodeMessageType] = &[NodeHello];

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ClientMessageType {
    ClientHello,
    ClientGet,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NodeMessageType {
    NodeHello,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MessageType {
    ClientMessageIdentifier(ClientMessageType),
    NodeMessageIdentifier(NodeMessageType),
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
        }
    }
}

impl MessageType {
    pub fn name(&self) -> &'static str {
        match self {
            MessageType::ClientMessageIdentifier(inner) => inner.name(),
            MessageType::NodeMessageIdentifier(inner) => inner.name(),
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
            .map(MessageType::NodeMessageIdentifier)
            .or(ClientMessageType::try_from(value).map(MessageType::ClientMessageIdentifier))
    }
}

impl MessageType {
    pub async fn decode(
        encoded: &mut PeekableReader<impl AsyncRead + Unpin>,
    ) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let mut peeker = Peeker::new(encoded);
        let peeked_identifier = peeker.next_contentful_line().await?.ok_or(UnexpectedEOF)?;

        let res = MessageType::try_from(peeked_identifier.deref())?;

        let stats = peeker.into();
        encoded.advance_to_peeker_stats(stats);

        Ok(res)
    }
}
