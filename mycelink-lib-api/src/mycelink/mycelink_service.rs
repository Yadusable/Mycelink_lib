use crate::db::db_connector::{DBConnector, DatabaseBackend};
use crate::model::chat::Chat;
use crate::model::contact::Contact;
use crate::model::messenger_service::MessengerService;
use crate::model::tenant::Tenant;
use futures::stream::BoxStream;

pub struct MycelinkService {
    db: DBConnector<Tenant>,
}

impl MessengerService for MycelinkService {
    fn list_chats(&self) -> BoxStream<dyn Chat> {
        todo!()
    }

    fn list_contacts(&self) -> BoxStream<dyn Contact> {
        todo!()
    }
}
