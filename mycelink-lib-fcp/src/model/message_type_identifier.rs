use crate::decode_error::DecodeError;
use crate::decode_error::DecodeError::UnexpectedEOF;
use crate::model::message_type_identifier::ClientMessageType::{
    ClientGet, ClientHello, ClientPut, GenerateSSK, ListPeer, TestDDARequest, TestDDAResponse,
};
use crate::model::message_type_identifier::MessageType::{Client, Node};
use crate::model::message_type_identifier::NodeMessageType::{
    AllData, DataFound, GetFailed, NodeHello, ProtocolError, PutFailed, PutSuccessful, SSKKeypair,
    TestDDAComplete, TestDDAReply, URIGenerated,
};
use crate::peekable_reader::Peeker;
use std::ops::Deref;
use tokio::io::AsyncRead;

pub const CLIENT_MESSAGE_TYPES: &[ClientMessageType] = &[
    ClientHello,
    ClientGet,
    ClientPut,
    ListPeer,
    GenerateSSK,
    TestDDARequest,
    TestDDAResponse,
];
pub const NODE_MESSAGE_TYPES: &[NodeMessageType] = &[
    NodeHello,
    AllData,
    PutSuccessful,
    PutFailed,
    GetFailed,
    URIGenerated,
    SSKKeypair,
    TestDDAReply,
    TestDDAComplete,
    DataFound,
    ProtocolError,
];

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ClientMessageType {
    ClientHello,
    ClientGet,
    ClientPut,
    ListPeer,
    GenerateSSK,
    TestDDARequest,
    TestDDAResponse,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NodeMessageType {
    NodeHello,
    AllData,
    PutSuccessful,
    PutFailed,
    GetFailed,
    URIGenerated,
    SSKKeypair,
    TestDDAReply,
    TestDDAComplete,
    DataFound,
    ProtocolError,
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
            AllData => "AllData",
            PutSuccessful => "PutSuccessful",
            URIGenerated => "URIGenerated",
            SSKKeypair => "SSKKeypair",
            TestDDAReply => "TestDDAReply",
            TestDDAComplete => "TestDDAComplete",
            DataFound => "DataFound",
            PutFailed => "PutFailed",
            GetFailed => "GetFailed",
            ProtocolError => "ProtocolError",
        }
    }
}

impl ClientMessageType {
    /// Gets the str representing the MessageIdentifier (e.g. "ClientHello")
    pub fn name(&self) -> &'static str {
        match self {
            ClientHello => "ClientHello",
            ClientGet => "ClientGet",
            ClientPut => "ClientPut",
            ListPeer => "ListPeer",
            GenerateSSK => "GenerateSSK",
            TestDDARequest => "TestDDARequest",
            TestDDAResponse => "TestDDAResponse",
        }
    }
}

impl MessageType {
    pub fn name(&self) -> &'static str {
        match self {
            Client(inner) => inner.name(),
            Node(inner) => inner.name(),
        }
    }

    pub fn is_specific_node_message(&self, matches: NodeMessageType) -> bool {
        match self {
            Client(_) => false,
            Node(inner) => inner == &matches,
        }
    }

    pub fn expect_specific_node_message(
        &self,
        matches: NodeMessageType,
    ) -> Result<(), DecodeError> {
        if !self.is_specific_node_message(matches) {
            Err(DecodeError::ExpectedDifferentMessageType {
                expected: Node(matches),
                got: *self,
            })
        } else {
            Ok(())
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
