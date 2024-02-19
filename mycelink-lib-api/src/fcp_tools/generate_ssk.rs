use mycelink_lib_fcp::decode_error::DecodeError;
use mycelink_lib_fcp::fcp_connector::{filters, FCPConnector, Listener};
use mycelink_lib_fcp::messages::generate_ssk::GenerateSSKMessage;
use mycelink_lib_fcp::messages::ssk_keypair::SSKKeypairMessage;
use mycelink_lib_fcp::model::message_type_identifier::NodeMessageType;
use mycelink_lib_fcp::model::unique_identifier::UniqueIdentifier;
use std::error::Error;
use std::fmt::{Display, Formatter};
use tokio::sync::mpsc;

pub async fn generate_ssk(
    fcp_connector: &FCPConnector,
) -> Result<SSKKeypairMessage, GenerateSSKKeypairError> {
    let identifier = UniqueIdentifier::new("generate_ssk");

    let (notifier, mut waiter) = mpsc::channel(1);
    let listener = Listener::new(
        vec![
            filters::identity_filter(identifier.clone()),
            filters::type_filter(NodeMessageType::SSKKeypair),
        ],
        Listener::DEFAULT_PRIORITY,
        notifier,
    );

    fcp_connector.add_listener(listener).await;

    let generate_message = GenerateSSKMessage { identifier };
    fcp_connector.send(generate_message).await?;

    match waiter.recv().await {
        None => Err(GenerateSSKKeypairError::Internal()),
        Some(message) => Ok(message.try_into()?),
    }
}

#[derive(Debug)]
pub enum GenerateSSKKeypairError {
    Internal(),
    Tokio { inner: tokio::io::Error },
    FCP { inner: DecodeError },
}

impl Display for GenerateSSKKeypairError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GenerateSSKKeypairError::Internal() => {
                write!(f, "Internal error")
            }
            GenerateSSKKeypairError::Tokio { inner } => {
                write!(f, "Tokio IO error ({inner})")
            }
            GenerateSSKKeypairError::FCP { inner } => {
                write!(f, "FCP Decode error ({inner})")
            }
        }
    }
}

impl Error for GenerateSSKKeypairError {}

impl From<tokio::io::Error> for GenerateSSKKeypairError {
    fn from(value: tokio::io::Error) -> Self {
        GenerateSSKKeypairError::Tokio { inner: value }
    }
}

impl From<DecodeError> for GenerateSSKKeypairError {
    fn from(value: DecodeError) -> Self {
        GenerateSSKKeypairError::FCP { inner: value }
    }
}

#[cfg(test)]
mod tests {
    use crate::fcp_tools::generate_ssk::generate_ssk;
    use mycelink_lib_fcp::fcp_connector::FCPConnector;
    use std::sync::Arc;
    use tokio::net::TcpStream;

    #[tokio::test]
    pub async fn test_generate_ssk() {
        let _ = env_logger::try_init();

        let stream = TcpStream::connect("localhost:9481").await.unwrap();
        let connector = Arc::new(
            FCPConnector::new(stream, "generate ssk test")
                .await
                .unwrap(),
        );
        let listen_connector = connector.clone();

        let _handle = tokio::spawn(async move { listen_connector.listen().await });

        let keypair = generate_ssk(&connector).await.unwrap();
    }
}
