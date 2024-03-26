use crate::model::chat::Chat;
use crate::model::contact::Contact;
use futures::stream::BoxStream;

pub trait MessengerService {
    fn list_chats(&self) -> BoxStream<dyn Chat>;
    fn list_contacts(&self) -> BoxStream<dyn Contact>;
}
