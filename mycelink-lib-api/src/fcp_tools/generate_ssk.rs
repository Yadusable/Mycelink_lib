use mycelink_lib_fcp::decode_error::DecodeError;
use mycelink_lib_fcp::fcp_connector::{filters, FCPConnector, Listener};
use mycelink_lib_fcp::messages::generate_ssk::GenerateSSKMessage;
use mycelink_lib_fcp::messages::ssk_keypair::SSKKeypairMessage;
use mycelink_lib_fcp::model::message_type_identifier::NodeMessageType;
use mycelink_lib_fcp::model::unique_identifier::UniqueIdentifier;
use std::error::Error;
use std::fmt::{Display, Formatter};

pub async fn generate_ssk(
    fcp_connector: &FCPConnector,
) -> Result<SSKKeypairMessage, GenerateSSKKeypairError> {
    let identifier = UniqueIdentifier::new("generate_ssk");

    let (listener, mut receiver) = Listener::new(
        vec![
            filters::identity_filter(identifier.clone()),
            filters::type_filter(NodeMessageType::SSKKeypair),
        ],
        Listener::DEFAULT_PRIORITY,
    );

    fcp_connector.add_listener(listener).await;

    let generate_message = GenerateSSKMessage { identifier };
    fcp_connector.send(generate_message).await?;

    match receiver.recv().await {
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
    use crate::test::create_test_fcp_connector;
    use std::sync::Arc;

    #[tokio::test]
    pub async fn test_generate_ssk() {
        let connector =
            Arc::new(create_test_fcp_connector("generate_ssk::test_generate_ssk").await);

        let keypair = generate_ssk(&connector).await.unwrap();
        assert_ne!(keypair.request_uri, keypair.insert_uri);
    }
}
