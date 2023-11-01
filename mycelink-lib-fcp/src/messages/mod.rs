pub mod client_hello;
pub mod node_hello;

use crate::decode_error::DecodeError;
use crate::fcp_parser::FCPParser;
use crate::messages::client_hello::ClientHelloMessage;
use crate::peekable_reader::PeekableReader;
use async_trait::async_trait;
use tokio::io::{AsyncRead, BufReader};

pub enum Message {
    ClientHello(ClientHelloMessage),
}

impl Message {
    pub fn encode(&self) -> Vec<u8> {
        todo!()
    }

    pub fn decode_reader(reader: &mut PeekableReader<impl AsyncRead + Unpin>) -> Message {
        todo!()
    }
}

pub struct MessagePayload {}

impl MessagePayload {
    pub fn decode_reader(reader: &mut PeekableReader<impl AsyncRead + Unpin>) -> Message {
        todo!()
    }
}

#[async_trait]
pub trait FCPEncodable {
    fn encode(&self) -> String;

    async fn decode(
        encoded: &mut FCPParser<BufReader<impl AsyncRead + Unpin + Send>>,
    ) -> Result<Self, DecodeError>
    where
        Self: Sized;
}
