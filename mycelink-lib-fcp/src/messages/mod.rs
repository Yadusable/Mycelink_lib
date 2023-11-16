pub mod client_get;
pub mod client_hello;
pub mod node_hello;

use crate::decode_error::DecodeError;
use crate::fcp_parser::FCPParser;
use crate::messages::client_hello::ClientHelloMessage;
use crate::messages::node_hello::NodeHelloMessage;
use crate::model::message_type_identifier::NodeMessageType;
use crate::peekable_reader::PeekableReader;
use async_trait::async_trait;
use tokio::io::AsyncRead;

pub enum ClientMessage {
    ClientHello(ClientHelloMessage),
}

impl ClientMessage {
    pub fn encode(&self) -> Vec<u8> {
        match self {
            ClientMessage::ClientHello(inner) => inner.encode().into_bytes(),
        }
    }
}

pub enum NodeMessage {
    NodeHello(NodeHelloMessage),
}

impl NodeMessage {
    pub async fn decode_reader(
        reader: &mut FCPParser<'_, impl AsyncRead + Unpin + Send>,
    ) -> Result<NodeMessage, DecodeError> {
        match reader.peek_node_identifier().await? {
            NodeMessageType::NodeHello => Ok(NodeMessage::NodeHello(
                NodeHelloMessage::decode(reader).await?,
            )),
        }
    }
}

pub struct MessagePayload {}

impl MessagePayload {
    pub fn decode_reader(reader: &mut PeekableReader<impl AsyncRead + Unpin>) -> MessagePayload {
        todo!()
    }
}

#[async_trait]
pub trait FCPEncodable {
    fn encode(&self) -> String;
}

#[async_trait]
pub trait FCPDecodable {
    async fn decode(
        encoded: &mut FCPParser<impl AsyncRead + Unpin + Send>,
    ) -> Result<Self, DecodeError>
    where
        Self: Sized;
}
