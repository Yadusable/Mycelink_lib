use crate::db::actions::chat_actions::ChatId;
use crate::model::message::Message;

pub trait MessengerService {
    fn send_message(&self, message: Message, chat_id: ChatId);
}
