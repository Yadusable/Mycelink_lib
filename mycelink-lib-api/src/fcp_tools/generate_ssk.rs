use mycelink_lib_fcp::fcp_connector::{filters, FCPConnector, Listener};
use mycelink_lib_fcp::messages::generate_ssk::GenerateSSKMessage;
use mycelink_lib_fcp::messages::ssk_keypair::SSKKeypairMessage;
use mycelink_lib_fcp::model::message_type_identifier::NodeMessageType;
use mycelink_lib_fcp::model::unique_identifier::UniqueIdentifier;
use tokio::sync::oneshot;

pub async fn generate_ssk(
    fcp_connector: &FCPConnector<'_>,
) -> Result<SSKKeypairMessage, tokio::io::Error> {
    let identifier = UniqueIdentifier::new("generate_ssk");

    let (res_tx, res_rx) = oneshot::channel();
    let listener = Listener::new(
        vec![
            filters::identity_filter(identifier),
            filters::type_filter(NodeMessageType::SSKKeypair),
        ],
        Listener::DEFAULT_PRIORITY,
        |message| {
            res_tx.send(message);
            true
        },
    );

    let generate_message = GenerateSSKMessage { identifier };
    fcp_connector.send(generate_message).await?;
}
