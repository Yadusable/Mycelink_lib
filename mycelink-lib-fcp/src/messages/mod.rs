mod client_hello;

use tokio::io::{AsyncRead, BufReader};

pub enum Message {
    ClientHello
}

impl Message {

    pub fn encode(&self) -> Vec<u8> {
        todo!()
    }

    pub fn decode_reader(reader: &mut BufReader<impl AsyncRead>) -> Message {
        todo!()
    }

}

pub struct MessagePayload {}

impl MessagePayload {
    pub fn decode_reader(reader: &mut BufReader<impl AsyncRead>) -> Message {
        todo!()
    }
}

pub trait FCPEncodable {

    fn encode(&self) -> String;

    fn decode(encoded: String) -> Self;

}