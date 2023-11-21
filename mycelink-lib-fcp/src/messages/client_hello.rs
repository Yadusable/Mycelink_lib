use crate::messages::FCPEncodable;
use crate::model::fcp_version::FCPVersion;
use crate::model::fields::Field;
use crate::model::message::Message;
use crate::model::message_type_identifier::ClientMessageType::ClientHello;
use crate::model::message_type_identifier::MessageType;
use async_trait::async_trait;

pub const EXPECTED_VERSION: FCPVersion = FCPVersion::V2_0;
const MESSAGE_TYPE: MessageType = MessageType::Client(ClientHello);

pub struct ClientHelloMessage {
    pub name: Box<str>,
    pub version: FCPVersion,
}

impl From<ClientHelloMessage> for Message {
    fn from(value: ClientHelloMessage) -> Self {
        Message::new(
            MESSAGE_TYPE,
            vec![
                Field::new("Name".into(), value.name),
                Field::new("ExpectedVersion".into(), value.version.name().into()),
            ]
            .into(),
            None,
        )
    }
}

#[async_trait]
impl FCPEncodable for ClientHelloMessage {
    fn encode(&self) -> String {
        let mut builder = String::new();

        builder.push_str(MESSAGE_TYPE.name());
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
    use crate::messages::FCPEncodable;
    use crate::model::message::Message;

    #[test]
    fn test_encode() {
        let client_hello = ClientHello(ClientHelloMessage {
            version: EXPECTED_VERSION,
            name: "Encode-Test".into(),
        });

        let encoded = Into::<Message>::into(client_hello).encode();

        assert_eq!(
            encoded.as_str(),
            "ClientHello\nName=Encode-Test\nExpectedVersion=2.0\nEndMessage\n"
        )
    }
}
