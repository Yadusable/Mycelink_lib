use crate::db::actions::chat_actions::ChatId;
use crate::db::actions::tenant_actions::Tenant;
use crate::db::db_connector::DBConnector;
use crate::fcp_tools::fcp_get::fcp_get_inline;
use crate::model::chat_config::ChatConfig::Mycelink;
use crate::model::connection_details::{PublicConnectionDetails, PublicMycelinkConnectionDetails};
use crate::model::message::ProtocolMessageMeta;
use crate::model::message_types::MessageType;
use crate::model::messenger_service::{MessengerService, PollError, SendMessageError};
use crate::model::protocol_config::Protocol;
use crate::mycelink::mycelink_account::MycelinkAccount;
use crate::mycelink::mycelink_chat::{MycelinkChat, MycelinkChatType};
use crate::mycelink::mycelink_contact::MycelinkContact;
use crate::mycelink::protocol::mycelink_channel_request::EncryptedSignedMycelinkChannelRequest;
use futures::StreamExt;
use mycelink_lib_fcp::fcp_connector::FCPConnector;
use mycelink_lib_fcp::model::priority_class::PriorityClass;
use mycelink_lib_fcp::model::uri::URI;
use std::future::Future;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct MycelinkService {
    db: DBConnector<Tenant>,
    fcp_connector: Arc<FCPConnector>,
    account: Mutex<MycelinkAccount>,
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
            account: Mutex::new(account),
        }
    }

    pub async fn poll(&self) -> Result<(), PollError> {
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

    async fn poll_incoming_channel_requests(&self) -> Result<(), PollError> {
        let mut has_updated = false;
        let mut account = self.account.lock().await;
        loop {
            let get_uri: URI = format!(
                "{}-{}",
                account.channel_request_dropbox_request_key(),
                account.channel_request_dropbox_known()
            )
            .as_str()
            .try_into()
            .unwrap();

            if let Err(_) = self
                .poll_single_incoming_channel_request(get_uri, &*account)
                .await
            {
                break;
            }
            account.channel_request_dropbox_known += 1;
            has_updated = true;
        }

        self.db
            .update_mycelink_account(account.deref().clone())
            .await?;
        Ok(())
    }

    async fn poll_single_incoming_channel_request(
        &self,
        uri: URI,
        account: &MycelinkAccount,
    ) -> Result<(), PollError> {
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
                let (request, _signing_key) = encrypted_request.try_open(account)?;
                //TODO verify signing key
                //TODO no just no

                let display_name;
                let public_connection_details: PublicMycelinkConnectionDetails;

                if let Some((contact, display)) = self
                    .db
                    .get_mycelink_connection_contact_by_request_key(request.contact_request_key())
                    .await?
                {
                    display_name = display.display_name;
                    public_connection_details = contact.into();
                } else {
                    display_name = "Unverified Contact".into();
                    let fetched = fcp_get_inline(
                        request.contact_request_key().try_into().unwrap(),
                        self.fcp_connector.deref(),
                        "Get public info of incoming channel",
                        PriorityClass::Medium,
                    )
                    .await?;
                    public_connection_details = ciborium::from_reader(fetched.data.as_ref())?;

                    self.db
                        .add_contact(
                            &PublicConnectionDetails::Mycelink(public_connection_details.clone()),
                            display_name.as_ref(),
                            None,
                            None,
                        )
                        .await?;
                }

                let channel = account
                    .accept_channel_request(request, self.fcp_connector.deref())
                    .await?;

                let chat = MycelinkChat {
                    chat_type: MycelinkChatType::DirectChat {
                        channel,
                        contact: MycelinkContact::new(
                            display_name.clone(),
                            public_connection_details,
                        ),
                    },
                };

                self.db
                    .create_chat(display_name.as_ref(), chat.into())
                    .await?;

                Ok(())
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
