use crate::crypto::kdf_provider::KdfProviderTag;
use crate::crypto::tagged_types::tagged_key_exchange::{
    TaggedAnswerKeyExchange, TaggedInitiateKeyExchange,
};
use crate::mycelink::compressed_box::{CompressionHint, CompressionHinting};
use crate::mycelink::mycelink_chat_message::MycelinkChatMessage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InitialChannelMessage {
    pub available_public_component: Box<[TaggedInitiateKeyExchange]>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MycelinkChannelMessage {
    GroupChatRekey {
        //TODO
    },
    FinalMessage {
        new_key: TaggedAnswerKeyExchange,
        new_kdf: KdfProviderTag,
        next_public_components: Box<[TaggedInitiateKeyExchange]>,

        attached_message: Box<MycelinkChannelMessage>,
    },
    DirectMessage(MycelinkChatMessage),
}

impl CompressionHinting for MycelinkChannelMessage {
    fn compression_hint(&self) -> CompressionHint {
        match self {
            MycelinkChannelMessage::GroupChatRekey { .. } => CompressionHint::Fast,
            MycelinkChannelMessage::FinalMessage { .. } => CompressionHint::Fast,
            MycelinkChannelMessage::DirectMessage(inner) => inner.compression_hint(),
        }
    }
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
