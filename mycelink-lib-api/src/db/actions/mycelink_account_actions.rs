use crate::db::db_connector::{DBConnector, DatabaseBackend};
use crate::model::mycelink_account::MycelinkAccount;
use crate::model::tenant::Tenant;
use sqlx::{Row, Transaction};
use std::error::Error;
use std::fmt::{Display, Formatter};

impl DBConnector<Tenant> {
    pub async fn create_mycelink_account_entry(
        &self,
        tx: &mut Transaction<'_, DatabaseBackend>,
        account: &MycelinkAccount,
    ) -> Result<(), MycelinkAccountEntryError> {
        if self.get_mycelink_account(tx).await?.is_some() {
            return Err(MycelinkAccountEntryError::AccountAlreadyExists);
        }

        let account_json = serde_json::to_string(account)?;

        let query = sqlx::query(
            "INSERT INTO protocol_config_per_tenant (tenant, protocol, config) VALUES (?,?,?)",
        );
        query
            .bind(self.tenant().display_name())
            .bind("mycelink")
            .bind(account_json)
            .execute(&mut **tx)
            .await?;

        Ok(())
    }

    pub async fn get_mycelink_account(
        &self,
        tx: &mut Transaction<'_, DatabaseBackend>,
    ) -> Result<Option<MycelinkAccount>, MycelinkAccountEntryError> {
        let query = sqlx::query("SELECT (config) FROM protocol_config_per_tenant WHERE protocol = 'mycelink' AND tenant = ?");
        let res = query
            .bind(self.tenant().display_name())
            .fetch_optional(&mut **tx)
            .await?;

        if let Some(row) = res {
            let config: Box<str> = row.try_get("config")?;

            let account: MycelinkAccount = serde_json::from_str(&config)?;
            Ok(Some(account))
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug)]
pub enum MycelinkAccountEntryError {
    SqlxError { inner: sqlx::Error },
    SerdeJson { inner: serde_json::error::Error },
    AccountAlreadyExists,
}

impl Error for MycelinkAccountEntryError {}

impl Display for MycelinkAccountEntryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MycelinkAccountEntryError::SqlxError { inner } => inner.fmt(f),
            MycelinkAccountEntryError::SerdeJson { inner } => inner.fmt(f),
            MycelinkAccountEntryError::AccountAlreadyExists => {
                write!(f, "Account already exists")
            }
        }
    }
}

impl From<sqlx::Error> for MycelinkAccountEntryError {
    fn from(value: sqlx::Error) -> Self {
        MycelinkAccountEntryError::SqlxError { inner: value }
    }
}

impl From<serde_json::Error> for MycelinkAccountEntryError {
    fn from(value: serde_json::Error) -> Self {
        MycelinkAccountEntryError::SerdeJson { inner: value }
    }
}

#[cfg(test)]
mod tests {
    use crate::db::db_connector::DBConnector;
    use crate::model::mycelink_account::MycelinkAccount;

    #[tokio::test]
    async fn get_nonexistent_account() {
        let connector = DBConnector::new_testing().await.test_tenant().await;
        let mut tx = connector.begin().await.unwrap();

        assert_eq!(connector.get_mycelink_account(&mut tx).await.unwrap(), None);
        tx.commit().await.unwrap();
    }

    #[tokio::test]
    async fn create_and_get_account() {
        let connector = DBConnector::new_testing().await.test_tenant().await;
        let mut tx = connector.begin().await.unwrap();

        let account =
            MycelinkAccount::create_new("dummy request key".into(), "dummy insert key".into());

        connector
            .create_mycelink_account_entry(&mut tx, &account)
            .await
            .unwrap();

        let got_account = connector
            .get_mycelink_account(&mut tx)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(got_account, account);
        tx.commit().await.unwrap();
    }
}
