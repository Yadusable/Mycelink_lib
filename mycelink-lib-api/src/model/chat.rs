use crate::model::message::MessageId;
use crate::model::message_types::MessageType;
use futures::future::BoxFuture;
use futures::stream::BoxStream;
use mycelink_lib_fcp::model::message::Message;

pub trait Chat {
    fn send_message(&mut self, message_type: MessageType) -> BoxFuture<Result<(), Box<str>>>;

    fn open_message_streams_at(&self, message_id: MessageId) -> BoxStream<Message>;
    fn open_message_streams_newest(&self) -> BoxStream<Message>;

    fn display_name(&self) -> &str;
    fn alternative_name(&self) -> Option<&str>;
}
