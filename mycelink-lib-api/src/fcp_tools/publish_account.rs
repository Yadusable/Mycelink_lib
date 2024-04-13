use crate::fcp_tools::fcp_put::{fcp_put_inline, FcpPutError};
use crate::mycelink::mycelink_account::MycelinkAccount;
use mycelink_lib_fcp::fcp_connector::FCPConnector;
use mycelink_lib_fcp::model::uri::URI;
use std::error::Error;
use std::fmt::{Display, Formatter};

pub async fn publish_account(
    fcp_connector: &FCPConnector,
    account: &MycelinkAccount,
    display_name: impl Into<Box<str>>,
) -> Result<(), PublishAccountError> {
    let uri: URI = account.insert_ssk_key().try_into().unwrap();

    let mut encoded = Vec::new();
    ciborium::into_writer(&account.generate_contact_info(display_name), &mut encoded).unwrap();

    fcp_put_inline(encoded.into(), uri, fcp_connector, "Publish Account").await?;

    Ok(())
}

#[derive(Debug)]
pub enum PublishAccountError {
    PutFailed { inner: FcpPutError },
}

impl Error for PublishAccountError {}

impl Display for PublishAccountError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<FcpPutError> for PublishAccountError {
    fn from(value: FcpPutError) -> Self {
        Self::PutFailed { inner: value }
    }
}

#[cfg(test)]
mod tests {
    use crate::fcp_tools::fcp_get::fcp_get_inline;
    use crate::fcp_tools::generate_ssk::generate_ssk;
    use crate::fcp_tools::publish_account::publish_account;
    use crate::mycelink::mycelink_account::MycelinkAccount;
    use crate::test::create_test_fcp_connector;
    use mycelink_lib_fcp::model::priority_class::PriorityClass;

    #[tokio::test]
    pub async fn test_upload_download() {
        let _ = env_logger::try_init();
        let fcp_connector =
            create_test_fcp_connector("publish_account::test_upload_download").await;

        let ssk_key = generate_ssk(&fcp_connector).await.unwrap();
        let account = MycelinkAccount::create_new(ssk_key.request_uri, ssk_key.insert_uri);

        publish_account(&fcp_connector, &account, "Test Account")
            .await
            .unwrap();

        let get = fcp_get_inline(
            account.request_ssk_key().try_into().unwrap(),
            &fcp_connector,
            "publish_account:test_upload_download",
            PriorityClass::High,
        )
        .await
        .unwrap();

        let mut payload = Vec::new();
        ciborium::into_writer(&account.generate_contact_info("Test Account"), &mut payload)
            .unwrap();

        assert_eq!(get.data, payload.into())
    }
}
