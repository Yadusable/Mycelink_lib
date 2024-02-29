use crate::api::{APIConnector, LoginStatus};
use crate::db::actions::account_actions::MycelinkAccountEntryError;
use crate::fcp_tools::generate_ssk::{generate_ssk, GenerateSSKKeypairError};
use crate::fcp_tools::publish_account::{publish_account, PublishAccountError};
use crate::model::account::Account;
use crate::model::tenant::Tenant;
use std::error::Error;
use std::fmt::{Display, Formatter};

impl<L: LoginStatus> APIConnector<L, Tenant> {
    pub async fn create_account(
        &self,
        display_name: impl Into<Box<str>>,
    ) -> Result<Box<str>, CreateAccountError> {
        let ssk_key = generate_ssk(&self.fcp_connector).await?;

        let account = Account::create_new(ssk_key.request_uri, ssk_key.insert_uri);

        let mut tx = self.db_connector.begin().await?;
        self.db_connector
            .create_mycelink_account_entry(&mut tx, &account)
            .await?;

        publish_account(&self.fcp_connector, &account, display_name).await?;

        tx.commit().await?;

        Ok(account.request_ssk_key().into())
    }
}

#[derive(Debug)]
pub enum CreateAccountError {
    GenerateSSKKeyPairError { inner: GenerateSSKKeypairError },
    PublishAccountError { inner: PublishAccountError },
    DatabaseError { inner: sqlx::Error },
    MycelinkAccountEntryError { inner: MycelinkAccountEntryError },
}

impl Error for CreateAccountError {}

impl Display for CreateAccountError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CreateAccountError::GenerateSSKKeyPairError { inner } => write!(f, "{inner}"),
            CreateAccountError::PublishAccountError { inner } => write!(f, "{inner}"),
            CreateAccountError::DatabaseError { inner } => write!(f, "{inner}"),
            CreateAccountError::MycelinkAccountEntryError { inner } => write!(f, "{inner}"),
        }
    }
}

impl From<GenerateSSKKeypairError> for CreateAccountError {
    fn from(value: GenerateSSKKeypairError) -> Self {
        CreateAccountError::GenerateSSKKeyPairError { inner: value }
    }
}

impl From<sqlx::Error> for CreateAccountError {
    fn from(value: sqlx::Error) -> Self {
        Self::DatabaseError { inner: value }
    }
}

impl From<PublishAccountError> for CreateAccountError {
    fn from(value: PublishAccountError) -> Self {
        Self::PublishAccountError { inner: value }
    }
}

impl From<MycelinkAccountEntryError> for CreateAccountError {
    fn from(value: MycelinkAccountEntryError) -> Self {
        Self::MycelinkAccountEntryError { inner: value }
    }
}
