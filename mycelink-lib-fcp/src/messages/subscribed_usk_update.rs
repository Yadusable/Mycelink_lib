use crate::decode_error::DecodeError;
use crate::model::message::Message;
use crate::model::message_type_identifier::NodeMessageType::SubscribedUSKUpdate;
use crate::model::unique_identifier::UniqueIdentifier;
use crate::model::uri::URI;

pub struct SubscribedUSKUpdateMessage {
    pub identifier: UniqueIdentifier,
    pub edition: u64,
    pub uri: URI,
    pub new_known_good: bool,
    pub new_slot_true: bool,
}

impl TryFrom<Message> for SubscribedUSKUpdateMessage {
    type Error = DecodeError;

    fn try_from(value: Message) -> Result<Self, Self::Error> {
        value
            .message_type()
            .expect_specific_node_message(SubscribedUSKUpdate)?;

        Ok(SubscribedUSKUpdateMessage {
            identifier: value
                .fields()
                .get_or_err("Identifier")?
                .value()
                .try_into()?,
            edition: value.fields().get_or_err("Edition")?.value().parse()?,
            uri: value.fields().get_or_err("URI")?.value().try_into()?,
            new_known_good: value.fields().get_or_err("NewKnownGood")?.value().parse()?,
            new_slot_true: value.fields().get_or_err("NewSlotToo")?.value().parse()?,
        })
    }
}
