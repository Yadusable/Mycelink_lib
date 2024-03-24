use mycelink_lib_fcp::decode_error::DecodeError;
use mycelink_lib_fcp::fcp_connector::filters::{identity_filter, type_filter};
use mycelink_lib_fcp::fcp_connector::{FCPConnector, Listener};
use mycelink_lib_fcp::messages::all_data::AllDataMessage;
use mycelink_lib_fcp::messages::client_get::ClientGetMessage;
use mycelink_lib_fcp::messages::get_failed::GetFailedMessage;
use mycelink_lib_fcp::model::message_type_identifier::NodeMessageType::{AllData, GetFailed};
use mycelink_lib_fcp::model::persistence::Persistence;
use mycelink_lib_fcp::model::priority_class::PriorityClass;
use mycelink_lib_fcp::model::return_type::ReturnType;
use mycelink_lib_fcp::model::unique_identifier::UniqueIdentifier;
use mycelink_lib_fcp::model::uri::URI;
use std::fmt::{Display, Formatter};
use std::io::Error;

const CLIENT_GET_MAX_INLINE_SIZE: usize = 1024 * 1024 * 256; // 256 MiB

pub async fn fcp_get_inline(
    uri: URI,
    fcp_connector: &FCPConnector,
    purpose: &str,
    priority: PriorityClass,
) -> Result<AllDataMessage, FcpGetError> {
    let identifier = UniqueIdentifier::new(purpose);
    let client_get = ClientGetMessage {
        identifier: identifier.clone(),
        uri,
        verbosity: Default::default(),
        return_type: ReturnType::Direct,
        max_size: Some(CLIENT_GET_MAX_INLINE_SIZE),
        max_temp_size: None,
        max_retries: 0,
        priority,
        persistence: Persistence::Connection,
        ignore_data_store: false,
        data_store_only: false,
        real_time: true,
    };

    let (success_listener, mut success_rx) = Listener::new(
        vec![identity_filter(identifier.clone()), type_filter(AllData)],
        0,
    );

    let (failure_listener, mut failure_rx) =
        Listener::new(vec![identity_filter(identifier), type_filter(GetFailed)], 0);

    fcp_connector.add_listener(success_listener).await;
    fcp_connector.add_listener(failure_listener).await;

    fcp_connector.send(&client_get).await?;

    tokio::select! {
        success = success_rx.recv() => {
            Ok(success.unwrap().try_into()?)
        },
        failure = failure_rx.recv() => {
            Err(FcpGetError::GetFailed {inner: failure.unwrap().try_into()?})
        }
    }
}

#[derive(Debug)]
pub enum FcpGetError {
    GetFailed { inner: GetFailedMessage },
    TokioIo { inner: tokio::io::Error },
    DecodeError { inner: DecodeError },
}

impl Display for FcpGetError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FcpGetError::GetFailed { inner } => write!(f, "GetFailed: {inner:?}"),
            FcpGetError::TokioIo { inner } => {
                write!(f, "TokioIoError: {inner}")
            }
            FcpGetError::DecodeError { inner } => {
                write!(f, "DecodeError: {inner}")
            }
        }
    }
}

impl std::error::Error for FcpGetError {}

impl From<tokio::io::Error> for FcpGetError {
    fn from(value: Error) -> Self {
        Self::TokioIo { inner: value }
    }
}

impl From<DecodeError> for FcpGetError {
    fn from(value: DecodeError) -> Self {
        Self::DecodeError { inner: value }
    }
}
