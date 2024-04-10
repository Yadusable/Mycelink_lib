use crate::db::actions::chat_actions::ChatId;
use crate::db::actions::contact_actions::ContactId;
use crate::db::actions::tenant_actions::Tenant;
use crate::db::db_connector::DBConnector;
use crate::fcp_tools::fcp_put::FcpPutError;
use crate::model::contact::ContactDisplay;
use crate::model::message::{Message, ProtocolMessageMeta};
use crate::model::message_types::MessageType;
use crate::model::messenger_service::PollError;
use crate::mycelink::mycelink_contact::MycelinkContact;
use crate::mycelink::protocol::mycelink_channel::{MycelinkChannel, ReceiveMessageError};
use crate::mycelink::protocol::mycelink_channel_message::MycelinkChannelMessage;
use mycelink_lib_fcp::fcp_connector::FCPConnector;
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
    pub(crate) async fn fetch(
        &mut self,
        db: &DBConnector<Tenant>,
        fcp: &FCPConnector,
        direct_chat_id: ChatId,
    ) -> Result<(), PollError> {
        match &mut self.chat_type {
            MycelinkChatType::DirectChat { channel, contact } => {
                while let Some(message) = channel.try_receive_message(fcp).await? {
                    match message {
                        MycelinkChannelMessage::GroupChatRekey { .. } => {
                            todo!()
                        }
                        MycelinkChannelMessage::FinalMessage { .. } => {
                            panic!("unreachable")
                        }
                        MycelinkChannelMessage::DirectMessage(message) => {
                            db.store_message(
                                db.mycelink_contact_id_to_contact_id(contact)
                                    .await
                                    .unwrap()
                                    .unwrap(),
                                &message.message_type().to_message_type(db).await,
                                (&message).into(),
                                message.timestamp(),
                                direct_chat_id,
                            )
                            .await?;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub async fn send(
        &mut self,
        message: &MessageType,
        fcp_connector: &FCPConnector,
        db_connector: &DBConnector<Tenant>,
    ) -> Result<ProtocolMessageMeta, FcpPutError> {
        let id = match &mut self.chat_type {
            MycelinkChatType::DirectChat { channel, .. } => {
                channel
                    .send_chat_message(message.as_mycelink(db_connector).await, fcp_connector)
                    .await?
            }
        };

        Ok(ProtocolMessageMeta::Mycelink { id })
    }
}
