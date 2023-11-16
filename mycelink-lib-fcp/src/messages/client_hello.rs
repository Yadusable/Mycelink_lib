use crate::messages::FCPEncodable;
use crate::model::fcp_version::FCPVersion;
use crate::model::message_type_identifier::ClientMessageType;
use async_trait::async_trait;

pub const EXPECTED_VERSION: FCPVersion = FCPVersion::V2_0;
const IDENTIFIER: ClientMessageType = ClientMessageType::ClientHello;

pub struct ClientHelloMessage {
    pub name: Box<str>,
    pub version: FCPVersion,
}

#[async_trait]
impl FCPEncodable for ClientHelloMessage {
    fn encode(&self) -> String {
        let mut builder = String::new();

        builder.push_str(IDENTIFIER.name());
        builder.push_str("\nName=");
        builder.push_str(&self.name);
        builder.push_str("\nExpectedVersion=");
        builder.push_str(self.version.name());
        builder.push_str("\nEndMessage\n");

        builder
    }
}

#[cfg(test)]
mod tests {
    use crate::messages::client_hello::{ClientHelloMessage, EXPECTED_VERSION};
    use crate::messages::ClientMessage::ClientHello;

    #[test]
    fn test_encode() {
        let client_hello = ClientHello(ClientHelloMessage {
            version: EXPECTED_VERSION,
            name: "Encode-Test".into(),
        });

        let encoded = client_hello.encode();

        let str = String::from_utf8(encoded).unwrap();

        assert_eq!(
            str.as_str(),
            "ClientHello\nName=Encode-Test\nExpectedVersion=2.0\nEndMessage\n"
        )
    }
}
