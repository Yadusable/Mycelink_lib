use crate::db::actions::message_actions::MessageId;
use crate::model::media::MediaId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum MessageType {
    Standard {
        content: MessageContent,
    },
    Reply {
        thread_start: MessageId,
        content: MessageContent,
    },
    Reaction {
        target_message: MessageId,
        indicator: char,
    },
}

#[derive(Serialize, Deserialize)]
pub enum MessageContent {
    Text {
        content: Box<str>,
    },
    Media {
        mime_type: Box<str>,
        media_size: u64,
        media_id: MediaId,
        filename: Box<str>,
    },
}
