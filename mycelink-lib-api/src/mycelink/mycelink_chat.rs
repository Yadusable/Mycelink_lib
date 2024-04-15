use crate::crypto::signed_box::SignedBox;
use crate::crypto::tagged_types::keys::KeyOrderExt;
use crate::db::actions::chat_actions::ChatId;
use crate::db::actions::tenant_actions::Tenant;
use crate::db::db_connector::DBConnector;
use crate::fcp_tools::fcp_put::{fcp_put_inline, FcpPutError};
use crate::model::message::ProtocolMessageMeta;
use crate::model::message_types::MessageType;
use crate::model::messenger_service::PollError;
use crate::mycelink::mycelink_account::MycelinkAccount;
use crate::mycelink::mycelink_contact::MycelinkContact;
use crate::mycelink::protocol::mycelink_channel::MycelinkChannel;
use crate::mycelink::protocol::mycelink_channel_message::MycelinkChannelMessage;
use crate::mycelink::protocol::mycelink_channel_request::{
    MycelinkChannelRequest, OpenChannelError,
};
use mycelink_lib_fcp::fcp_connector::FCPConnector;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MycelinkChat {
    pub chat_type: MycelinkChatType,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MycelinkChatType {
    DirectChat {
        channel: MycelinkChannel,
        contact: MycelinkContact,
    },
}

impl MycelinkChat {
    pub async fn new_direct_chat(
        account: &MycelinkAccount,
        contact: MycelinkContact,
        fcp: &FCPConnector,
    ) -> Result<Self, OpenChatError> {
        let recipient_pub_key = contact
            .connection_details()
            .public_encryption_keys()
            .iter()
            .get_recommended_key()
            .ok_or(OpenChatError::NoValidKey)?;

        let (request, channel) =
            MycelinkChannelRequest::create(account, recipient_pub_key.clone(), fcp).await?;

        let signed_request = request.sign(account);
        let encrypted_request = signed_request.encrypt(recipient_pub_key);

        let mut request_data = Vec::new();
        ciborium::into_writer(&encrypted_request, &mut request_data).unwrap();
        fcp_put_inline(
            request_data.into(),
            contact
                .connection_details()
                .channel_request_droppoint()
                .as_ref()
                .try_into()
                .unwrap(),
            fcp,
            "send channel request",
        )
        .await?;

        Ok(Self {
            chat_type: MycelinkChatType::DirectChat { channel, contact },
        })
    }

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

    pub fn display_name(&self) -> &str {
        match &self.chat_type {
            MycelinkChatType::DirectChat { contact, .. } => contact.display_name(),
        }
    }
}

#[derive(Debug)]
pub enum OpenChatError {
    ContactDoesntExist,
    ContactIsNotMycelink,
    NoValidKey,
    Sqlx(sqlx::Error),
    OpenChannelError(OpenChannelError),
    FcpPutError(FcpPutError),
}

impl From<FcpPutError> for OpenChatError {
    fn from(value: FcpPutError) -> Self {
        Self::FcpPutError(value)
    }
}

impl From<sqlx::Error> for OpenChatError {
    fn from(value: sqlx::Error) -> Self {
        Self::Sqlx(value)
    }
}

impl From<OpenChannelError> for OpenChatError {
    fn from(value: OpenChannelError) -> Self {
        Self::OpenChannelError(value)
    }
}
