use crate::model::tagged_key_exchange::{TaggedAnswerKeyExchange, TaggedInitiateKeyExchange};
use crate::mycelink::mycelink_chat_message::MycelinkChatMessage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InitialChannelMessage {
    pub available_public_component: Box<[TaggedInitiateKeyExchange]>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MycelinkChannelMessage {
    GroupChatRekey {
        //TODO
    },
    FinalMessage {
        answer: TaggedAnswerKeyExchange,
    },
    DirectMessage(MycelinkChatMessage),
}

impl<'a> TryFrom<&'a MycelinkChannelMessage> for &'a MycelinkChatMessage {
    type Error = ();

    fn try_from(value: &'a MycelinkChannelMessage) -> Result<Self, Self::Error> {
        match value {
            MycelinkChannelMessage::DirectMessage(inner) => Ok(inner),
            _ => Err(()),
        }
    }
}
