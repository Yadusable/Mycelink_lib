use crate::model::chat::Chat;
use crate::model::contact::Contact;
use futures::Stream;

pub trait MessengerService {
    fn list_chats(&self) -> dyn Stream<Item = dyn Chat>;
    fn list_contacts(&self) -> dyn Stream<Item = dyn Contact>;
}
