mod client_hello;

use crate::decode_error::DecodeError;
use crate::peekable_reader::PeekableReader;
use async_trait::async_trait;
use tokio::io::AsyncRead;
use crate::messages::client_hello::ClientHelloMessage;

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
        encoded: &mut PeekableReader<impl AsyncRead + Unpin + Send>,
    ) -> Result<Self, DecodeError>
    where
        Self: Sized;
    
}
