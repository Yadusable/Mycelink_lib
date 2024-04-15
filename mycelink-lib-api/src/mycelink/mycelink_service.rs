use crate::db::actions::chat_actions::ChatId;
use crate::db::actions::tenant_actions::Tenant;
use crate::db::db_connector::DBConnector;
use crate::fcp_tools::fcp_get::{fcp_get_inline, FcpGetError};
use crate::model::chat_config::ChatConfig::Mycelink;
use crate::model::message::ProtocolMessageMeta;
use crate::model::message_types::MessageType;
use crate::model::messenger_service::{MessengerService, PollError, SendMessageError};
use crate::model::protocol_config::Protocol;
use crate::mycelink::mycelink_account::MycelinkAccount;
use crate::mycelink::mycelink_chat::MycelinkChat;
use crate::mycelink::protocol::mycelink_channel_request::EncryptedSignedMycelinkChannelRequest;
use futures::StreamExt;
use mycelink_lib_fcp::fcp_connector::FCPConnector;
use mycelink_lib_fcp::messages::all_data::AllDataMessage;
use mycelink_lib_fcp::model::priority_class::PriorityClass;
use mycelink_lib_fcp::model::uri::URI;
use sqlx::testing::TestTermination;
use std::future::Future;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;

#[derive(Clone)]
pub struct MycelinkService {
    db: DBConnector<Tenant>,
    fcp_connector: Arc<FCPConnector>,
    account: MycelinkAccount,
}

impl MycelinkService {
    async fn send_message_(
        &self,
        message: &MessageType,
        chat_id: ChatId,
    ) -> Result<ProtocolMessageMeta, SendMessageError> {
        let details = self.db.get_chat_config(chat_id).await?.unwrap();
        let Mycelink(mut details) = details;
        Ok(details
            .send(message, self.fcp_connector.as_ref(), &self.db)
            .await?)
    }

    pub fn new(
        db_connector: DBConnector<Tenant>,
        fcp_connector: Arc<FCPConnector>,
        account: MycelinkAccount,
    ) -> Self {
        Self {
            db: db_connector,
            fcp_connector,
            account,
        }
    }

    pub async fn poll(&mut self) -> Result<(), PollError> {
        self.poll_incoming_channel_requests().await;

        let mut chats = self.db.list_protocol_chats(self).await;

        while let Some(chat) = chats.next().await {
            let (chat, config) = chat?;

            let mut mycelink_chat: MycelinkChat = config.try_into().unwrap();
            mycelink_chat
                .fetch(&self.db, self.fcp_connector.as_ref(), chat.id)
                .await?; //TODO get actual direct chat id
        }

        todo!("append messages to open streams")
    }

    async fn poll_incoming_channel_requests(&mut self) {
        let mut updated = false;
        loop {
            let get_uri: URI = format!(
                "{}-{}",
                self.account.channel_request_dropbox_request_key(),
                self.account.channel_request_dropbox_known()
            )
            .as_str()
            .try_into()
            .unwrap();

            if let Err(_) = self.poll_single_incoming_channel_request(get_uri).await {
                break;
            }

            updated = true;
        }

        //todo persist changes to account
    }

    async fn poll_single_incoming_channel_request(&self, uri: URI) -> Result<(), ()> {
        let get_res = fcp_get_inline(
            uri,
            self.fcp_connector.as_ref(),
            "Poll incoming Channel requests",
            PriorityClass::Medium,
        )
        .await;

        match get_res {
            Ok(get_res) => {
                let encrypted_request: EncryptedSignedMycelinkChannelRequest =
                    ciborium::from_reader(get_res.data.deref())?;
                let (request, _signing_key) = encrypted_request.try_open(&self.account)?;
                //TODO verify signing key
            }
            Err(_) => {
                todo!()
            }
        }
    }
}

impl MessengerService for MycelinkService {
    fn protocol(&self) -> Protocol {
        Protocol::Mycelink
    }

    fn send_message<'a>(
        &'a self,
        message: &'a MessageType,
        chat_id: ChatId,
    ) -> Pin<Box<dyn Future<Output = Result<ProtocolMessageMeta, SendMessageError>> + '_>> {
        Box::pin(self.send_message_(message, chat_id))
    }
}
