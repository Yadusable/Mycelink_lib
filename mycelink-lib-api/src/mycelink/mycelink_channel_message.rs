use crate::model::tagged_key_exchange::{TaggedAnswerKeyExchange, TaggedInitiateKeyExchange};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InitialResponderChannelMessage {
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
    DirectMessage(),
}
