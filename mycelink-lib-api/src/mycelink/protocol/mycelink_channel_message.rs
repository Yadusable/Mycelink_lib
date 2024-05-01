use crate::crypto::kdf_provider::KdfProviderTag;
use crate::crypto::tagged_types::tagged_key_exchange::{
    TaggedAnswerKeyExchange, TaggedInitiateKeyExchange,
};
use crate::mycelink::protocol::compressed_box::{CompressionHint, CompressionHinting};
use crate::mycelink::protocol::mycelink_chat_message::MycelinkChatMessage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InitialChannelMessage {
    pub available_public_component: Box<[TaggedInitiateKeyExchange]>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MycelinkChannelMessage<'a> {
    GroupChatRekey {
        //TODO
    },
    GroupChatInvite {},
    ChannelRekey {
        new_key: TaggedAnswerKeyExchange,
        new_kdf: KdfProviderTag,
        next_public_components: Box<[TaggedInitiateKeyExchange]>,

        attached_message: Box<MycelinkChannelMessage<'a>>,
    },
    DirectMessage(MycelinkChatMessage<'a>),
}

impl CompressionHinting for MycelinkChannelMessage<'_> {
    fn compression_hint(&self) -> CompressionHint {
        match self {
            MycelinkChannelMessage::GroupChatRekey { .. } => CompressionHint::Fast,
            MycelinkChannelMessage::ChannelRekey { .. } => CompressionHint::Fast,
            MycelinkChannelMessage::DirectMessage(inner) => inner.compression_hint(),
            MycelinkChannelMessage::GroupChatInvite { .. } => CompressionHint::Fast,
        }
    }
}

impl<'a, 'b> TryFrom<&'a MycelinkChannelMessage<'b>> for &'a MycelinkChatMessage<'b> {
    type Error = ();

    fn try_from(value: &'a MycelinkChannelMessage<'b>) -> Result<Self, Self::Error> {
        match value {
            MycelinkChannelMessage::DirectMessage(inner) => Ok(inner),
            _ => Err(()),
        }
    }
}
