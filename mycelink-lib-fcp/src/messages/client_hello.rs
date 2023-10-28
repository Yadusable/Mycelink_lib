use tokio::io::AsyncRead;
use crate::messages::FCPEncodable;

const EXPECTED_VERSION: &str = "2.0";

pub struct ClientHello {
    name: String,
    expected_version: String,
}

impl FCPEncodable for ClientHello {
    fn encode(&self) -> String {
        let mut builder = String::new();

        builder.push_str("ClientHello\n");
        builder.push_str("Name=");
        builder.push_str(self.name.as_str());
        builder.push_str("\nExpectedVersion=");
        builder.push_str(self.expected_version.as_str());
        builder.push_str("\nEndMessage\n");

        builder
    }

    fn decode(encoded: impl AsyncRead) -> Self {
        

        todo!()
    }
}