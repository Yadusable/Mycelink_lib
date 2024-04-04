use crate::db::actions::chat_actions::ChatId;
use crate::db::actions::contact_actions::ContactId;
use crate::db::actions::message_actions::MessageId;
use crate::db::actions::tenant_actions::Tenant;
use crate::db::db_connector::DBConnector;
use crate::model::contact::ContactDisplay;
use crate::model::message::Message;
use crate::model::message_types::MessageType;
use crate::model::messenger_service::{MessengerService, SendMessageError};
use futures::Stream;
use std::ops::Deref;
use std::time::UNIX_EPOCH;

pub struct Chat<'a, 'b> {
    id: ChatId,
    display_name: Box<str>,
    alt_name: Option<Box<str>>,
    message_service: &'a dyn MessengerService,
    db_connector: &'b DBConnector<Tenant>,
}

pub struct MessageStreams<
    A: Stream<Item = sqlx::Result<Message>>,
    B: Stream<Item = sqlx::Result<Message>>,
> {
    next_messages: A,
    previous_messages: B,
}

impl Chat<'_, '_> {
    pub async fn send_message(
        &mut self,
        message_type: MessageType,
        sender_contact: ContactDisplay, // Typically own contact
    ) -> Result<(), SendMessageError> {
        let meta = self
            .message_service
            .send_message(&message_type, self.id)
            .await?;

        let message = Message {
            sender: sender_contact,
            message_id: MessageId(i64::MAX),
            protocol_message_meta: meta,
            reactions: vec![],
            replies: vec![],
            timestamp: UNIX_EPOCH.elapsed().unwrap().as_secs(), //TODO maybe use same timestamp as sent one?
            content: message_type,
        };

        self.db_connector.store_message(message, self.id).await?;

        Ok(())
    }

    pub async fn open_message_streams_at(
        &self,
        message_id: MessageId,
    ) -> MessageStreams<
        impl Stream<Item = sqlx::Result<Message>> + '_,
        impl Stream<Item = sqlx::Result<Message>> + '_,
    > {
        let prev = self.db_connector.get_previous_messages(message_id).await;
        let next = self.db_connector.get_next_messages(message_id).await;

        MessageStreams {
            next_messages: next,
            previous_messages: prev,
        }
    }
    pub async fn open_message_streams_newest(
        &self,
        chat_id: ChatId,
    ) -> sqlx::Result<
        MessageStreams<
            impl Stream<Item = sqlx::Result<Message>> + '_,
            impl Stream<Item = sqlx::Result<Message>> + '_,
        >,
    > {
        let newest = self.db_connector.get_newest_message(chat_id).await?;
        Ok(self.open_message_streams_at(newest.message_id).await)
    }

    pub fn display_name(&self) -> &str {
        self.display_name.deref()
    }
    pub fn alternative_name(&self) -> Option<&str> {
        self.alt_name.as_ref().map(|e| e.as_ref())
    }
    pub fn profile_picture(&self) -> Box<[u8]> {
        todo!()
    }
    pub fn id(&self) -> ChatId {
        self.id
    }
    pub async fn last_message(&self) -> Message {
        todo!()
    }
}
