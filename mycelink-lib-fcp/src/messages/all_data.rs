use crate::decode_error::DecodeError;
use crate::model::content_type::ContentType;
use crate::model::message::Message;
use crate::model::message_type_identifier::NodeMessageType;
use crate::model::unique_identifier::UniqueIdentifier;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct AllDataMessage {
    pub identifier: UniqueIdentifier,
    pub content_type: ContentType,

    pub data: Box<[u8]>,
}

impl TryFrom<Message> for AllDataMessage {
    type Error = DecodeError;

    fn try_from(value: Message) -> Result<Self, Self::Error> {
        value
            .message_type()
            .expect_specific_node_message(NodeMessageType::AllData)?;

        Ok(Self {
            identifier: value
                .fields()
                .get_or_err("Identifier")?
                .value()
                .try_into()?,
            content_type: value
                .fields()
                .get_or_err("Metadata.ContentType")?
                .value()
                .parse()?,
            data: value.payload().ok_or(DecodeError::MissingPayload)?.data,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::messages::all_data::AllDataMessage;
    use crate::model::content_type::ContentType;
    use crate::model::fields::Field;
    use crate::model::message::{Message, MessagePayload};
    use crate::model::message_type_identifier::MessageType::Node;
    use crate::model::message_type_identifier::NodeMessageType::AllData;
    use crate::model::unique_identifier::UniqueIdentifier;
    use std::str::FromStr;

    #[test]
    fn test_parse() {
        let identifier = UniqueIdentifier::new("Test");

        let message = Message::new(
            Node(AllData),
            vec![
                Field::new("Identifier".into(), identifier.to_string().into()),
                Field::new(
                    "Metadata.ContentType".into(),
                    "text/plain;charset=utf8".into(),
                ),
            ]
            .into(),
            Some(MessagePayload {
                data: r"Hello World".into(),
                data_len_identifier: "DataLength".into(),
            }),
        );

        let parsed = AllDataMessage::try_from(message).unwrap();

        assert_eq!(
            parsed,
            AllDataMessage {
                data: r"Hello World".to_string().into_bytes(),
                content_type: ContentType::from_str("text/plain;charset=utf8").unwrap(),
                identifier,
            }
        )
    }
}
