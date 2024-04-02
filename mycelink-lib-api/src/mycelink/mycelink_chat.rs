use crate::db::actions::tenant_actions::Tenant;
use crate::db::db_connector::DBConnector;
use crate::fcp_tools::fcp_put::FcpPutError;
use crate::model::message_types::MessageType;
use crate::mycelink::mycelink_contact::MycelinkContact;
use crate::mycelink::protocol::mycelink_channel::MycelinkChannel;
use mycelink_lib_fcp::fcp_connector::FCPConnector;
use mycelink_lib_fcp::model::message::Message;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MycelinkChat {
    chat_type: MycelinkChatType,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MycelinkChatType {
    DirectChat {
        channel: MycelinkChannel,
        contact: MycelinkContact,
    },
}

impl MycelinkChat {
    fn fetch(&self, db: &DBConnector<Tenant>, fcp: &FCPConnector) {}

    pub async fn send(
        &mut self,
        message: MessageType,
        fcp_connector: &FCPConnector,
        db_connector: &DBConnector<Tenant>,
    ) -> Result<(), FcpPutError> {
        match &mut self.chat_type {
            MycelinkChatType::DirectChat { channel, .. } => {
                channel
                    .send_chat_message(message.into_mycelink(db_connector).await, fcp_connector)
                    .await?;
            }
        }
        Ok(())
    }
}
