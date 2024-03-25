use mycelink_lib_fcp::decode_error::DecodeError;
use mycelink_lib_fcp::fcp_connector::filters::{identity_filter, type_filter};
use mycelink_lib_fcp::fcp_connector::{FCPConnector, Listener};
use mycelink_lib_fcp::messages::client_put::ClientPutMessage;
use mycelink_lib_fcp::messages::put_failed::PutFailedMessage;
use mycelink_lib_fcp::messages::put_successful::PutSuccessfulMessage;
use mycelink_lib_fcp::model::message_type_identifier::NodeMessageType::{PutFailed, PutSuccessful};
use mycelink_lib_fcp::model::persistence::Persistence;
use mycelink_lib_fcp::model::priority_class::PriorityClass;
use mycelink_lib_fcp::model::unique_identifier::UniqueIdentifier;
use mycelink_lib_fcp::model::upload_type::UploadType;
use mycelink_lib_fcp::model::uri::URI;
use mycelink_lib_fcp::model::verbosity::Verbosity;
use std::fmt::{Display, Formatter};
use std::io::Error;

pub async fn fcp_put_inline(
    data: Box<[u8]>,
    uri: URI,
    fcp_connector: &FCPConnector,
    intent: &str,
) -> Result<PutSuccessfulMessage, FcpPutError> {
    let identifier = UniqueIdentifier::new(intent);

    let put_message = ClientPutMessage {
        uri,
        content_type: None,
        identifier: identifier.clone(),
        verbosity: Verbosity::default(),
        max_retries: 1,
        priority: PriorityClass::High,
        get_only_chk: false,
        dont_compress: false,
        persistence: Persistence::Connection,
        target_filename: None,
        upload_from: UploadType::Direct { data },
        is_binary_blob: false,
        real_time: false,
    };

    let (listener_success, mut success_rx) = Listener::new(
        vec![
            identity_filter(identifier.clone()),
            type_filter(PutSuccessful),
        ],
        10,
    );

    let (listener_failed, mut failure_rx) =
        Listener::new(vec![identity_filter(identifier), type_filter(PutFailed)], 0);

    fcp_connector.add_listener(listener_failed).await;
    fcp_connector.add_listener(listener_success).await;

    fcp_connector.send(&put_message).await?;

    tokio::select! {
        success = success_rx.recv() => {
            Ok(success.unwrap().try_into()?)
        },
        failure = failure_rx.recv() => {
            Err(FcpPutError::PutFailed{inner: failure.unwrap().try_into()?})?
        }
    }
}

#[derive(Debug)]
pub enum FcpPutError {
    PutFailed { inner: PutFailedMessage },
    TokioIo { inner: tokio::io::Error },
    DecodeError { inner: DecodeError },
}

impl Display for FcpPutError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FcpPutError::PutFailed { inner } => write!(f, "PutFailed: {inner:?}"),
            FcpPutError::TokioIo { inner } => {
                write!(f, "TokioIoError: {inner}")
            }
            FcpPutError::DecodeError { inner } => {
                write!(f, "DecodeError: {inner}")
            }
        }
    }
}

impl std::error::Error for FcpPutError {}

impl From<tokio::io::Error> for FcpPutError {
    fn from(value: Error) -> Self {
        Self::TokioIo { inner: value }
    }
}

impl From<DecodeError> for FcpPutError {
    fn from(value: DecodeError) -> Self {
        Self::DecodeError { inner: value }
    }
}
