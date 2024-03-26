use crate::model::message::MessageId;
use crate::model::message_types::MessageType;
use futures::Stream;
use mycelink_lib_fcp::model::message::Message;
use std::future::Future;

pub trait Chat {
    fn send_message(&mut self, message_type: MessageType) -> dyn Future<Output = Result<(), ()>>;

    fn open_message_streams_at(&self, message_id: MessageId) -> dyn Stream<Item = Message>;
    fn open_message_streams_newest(&self) -> dyn Stream<Item = Message>;
}
