use crate::fcp_tools::fcp_put::{fcp_put_inline, FcpPutError};
use crate::model::account::Account;
use mycelink_lib_fcp::fcp_connector::FCPConnector;
use mycelink_lib_fcp::model::uri::URI;
use std::error::Error;
use std::fmt::{Display, Formatter};

pub async fn publish_account(
    fcp_connector: &FCPConnector,
    account: &Account,
    display_name: impl Into<Box<str>>,
) -> Result<(), PublishAccountError> {
    let uri: URI = account.insert_ssk_key().try_into().unwrap();

    let encoded = serde_json::to_string(&account.generate_contact_info(display_name))?;

    fcp_put_inline(encoded.into_bytes(), uri, fcp_connector).await?;

    Ok(())
}

#[derive(Debug)]
pub enum PublishAccountError {
    PutFailed { inner: FcpPutError },
    SerdeJson { inner: serde_json::Error },
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

impl From<serde_json::Error> for PublishAccountError {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeJson { inner: value }
    }
}

#[cfg(test)]
mod tests {
    use crate::fcp_tools::fcp_get::fcp_get_inline;
    use crate::fcp_tools::generate_ssk::generate_ssk;
    use crate::fcp_tools::publish_account::publish_account;
    use crate::model::account::Account;
    use crate::test::create_test_fcp_connector;
    use mycelink_lib_fcp::model::priority_class::PriorityClass;

    #[tokio::test]
    pub async fn test_upload_download() {
        let fcp_connector =
            create_test_fcp_connector("publish_account::test_upload_download").await;

        let ssk_key = generate_ssk(&fcp_connector).await.unwrap();
        let account = Account::create_new(ssk_key.request_uri, ssk_key.insert_uri);

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

        assert_eq!(
            get.data,
            serde_json::to_string(&account.generate_contact_info("Test Account"))
                .unwrap()
                .into_bytes()
        )
    }
}
