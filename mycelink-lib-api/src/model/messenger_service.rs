use crate::db::actions::chat_actions::ChatId;
use crate::fcp_tools::fcp_put::FcpPutError;
use crate::model::message_types::MessageType;
use sqlx::Error;
use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;

pub trait MessengerService {
    fn send_message(
        &self,
        message: MessageType,
        chat_id: ChatId,
    ) -> Pin<Box<dyn Future<Output = Result<(), SendMessageError>> + '_>>;
}

pub enum SendMessageError {
    Sqlx(sqlx::error::Error),
    Protocol(Box<dyn Debug>),
}

impl From<sqlx::error::Error> for SendMessageError {
    fn from(value: Error) -> Self {
        Self::Sqlx(value)
    }
}

impl From<FcpPutError> for SendMessageError {
    fn from(value: FcpPutError) -> Self {
        Self::Protocol(Box::new(value))
    }
}
