use crate::db::actions::chat_actions::ChatId;
use crate::db::actions::tenant_actions::Tenant;
use crate::db::db_connector::DBConnector;
use crate::model::chat_config::ChatConfig::Mycelink;
use crate::model::message::Message;
use crate::model::messenger_service::MessengerService;

pub struct MycelinkService {
    db: DBConnector<Tenant>,
}

impl MessengerService for MycelinkService {
    fn send_message(&self, message: Message, chat_id: ChatId) {
        async {
            let details = self.db.get_chat_config(chat_id).await;
            if let Mycelink(details) = details {
            } else {
                panic!()
            }
        }
    }
}
