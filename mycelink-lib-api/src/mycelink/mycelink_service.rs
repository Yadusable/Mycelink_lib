use crate::db::actions::chat_actions::ChatId;
use crate::db::actions::tenant_actions::Tenant;
use crate::db::db_connector::DBConnector;
use crate::model::chat_config::ChatConfig::Mycelink;
use crate::model::message_types::MessageType;
use crate::model::messenger_service::{MessengerService, SendMessageError};
use mycelink_lib_fcp::fcp_connector::FCPConnector;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub struct MycelinkService {
    db: DBConnector<Tenant>,
    fcp_connector: Arc<FCPConnector>,
}

impl MycelinkService {
    async fn send_message_(
        &self,
        message: MessageType,
        chat_id: ChatId,
    ) -> Result<(), SendMessageError> {
        let details = self.db.get_chat_config(chat_id).await?.unwrap();
        let Mycelink(mut details) = details;
        Ok(details
            .send(message, self.fcp_connector.as_ref(), &self.db)
            .await?)
    }
}

impl MessengerService for MycelinkService {
    fn send_message(
        &self,
        message: MessageType,
        chat_id: ChatId,
    ) -> Pin<Box<dyn Future<Output = Result<(), SendMessageError>> + '_>> {
        Box::pin(self.send_message_(message, chat_id))
    }
}
