use crate::db::actions::chat_actions::ChatId;
use crate::fcp_tools::fcp_get::FcpGetError;
use crate::fcp_tools::fcp_put::FcpPutError;
use crate::model::message::ProtocolMessageMeta;
use crate::model::message_types::MessageType;
use crate::model::protocol_config::Protocol;
use crate::mycelink::mycelink_service::MycelinkService;
use crate::mycelink::protocol::mycelink_channel::ReceiveMessageError;
use crate::mycelink::protocol::mycelink_channel_request::OpenChannelError;
use ciborium::de::Error;
use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;

pub trait MessengerService {
    fn protocol(&self) -> Protocol;

    fn send_message<'a>(
        &'a self,
        message: &'a MessageType,
        chat_id: ChatId,
    ) -> Pin<Box<dyn Future<Output = Result<ProtocolMessageMeta, SendMessageError>> + '_>>;
}

pub enum PollableService {
    MycelinkService(MycelinkService),
}

impl PollableService {
    pub async fn poll(&self) -> Result<(), PollError> {
        match self {
            PollableService::MycelinkService(service) => service.poll().await,
        }
    }

    pub fn service(&self) -> &(dyn MessengerService + Send + Sync) {
        match self {
            PollableService::MycelinkService(service) => service,
        }
    }
}

pub enum PollError {
    Sqlx(sqlx::Error),
    Mycelink(ReceiveMessageError),
    Ciborium(ciborium::de::Error<std::io::Error>),
    OpenChannelError(OpenChannelError),
    FcpGetError(FcpGetError),
}

impl From<sqlx::error::Error> for PollError {
    fn from(value: sqlx::error::Error) -> Self {
        Self::Sqlx(value)
    }
}

impl From<ReceiveMessageError> for PollError {
    fn from(value: ReceiveMessageError) -> Self {
        Self::Mycelink(value)
    }
}

impl From<ciborium::de::Error<std::io::Error>> for PollError {
    fn from(value: Error<std::io::Error>) -> Self {
        Self::Ciborium(value)
    }
}

impl From<OpenChannelError> for PollError {
    fn from(value: OpenChannelError) -> Self {
        Self::OpenChannelError(value)
    }
}

impl From<FcpGetError> for PollError {
    fn from(value: FcpGetError) -> Self {
        Self::FcpGetError(value)
    }
}

pub enum SendMessageError {
    Sqlx(sqlx::error::Error),
    Protocol(Box<dyn Debug>),
}

impl From<sqlx::error::Error> for SendMessageError {
    fn from(value: sqlx::error::Error) -> Self {
        Self::Sqlx(value)
    }
}

impl From<FcpPutError> for SendMessageError {
    fn from(value: FcpPutError) -> Self {
        Self::Protocol(Box::new(value))
    }
}
