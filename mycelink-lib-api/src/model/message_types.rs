use crate::model::media::MediaId;
use crate::model::message::MessageId;
use mime::Mime;

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

pub enum MessageContent {
    Text {
        content: Box<str>,
    },
    Media {
        mime_type: Mime,
        media_size: u64,
        media_id: MediaId,
        filename: Box<str>,
    },
}
