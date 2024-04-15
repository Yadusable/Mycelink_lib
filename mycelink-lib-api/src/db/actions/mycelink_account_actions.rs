use crate::db::actions::tenant_actions::Tenant;
use crate::db::db_connector::{DBConnector, DatabaseBackend};
use crate::model::protocol_config::{Protocol, ProtocolConfig};
use crate::mycelink::mycelink_account::MycelinkAccount;
use sqlx::types::Json;
use sqlx::{Row, Transaction};
use std::error::Error;
use std::fmt::{Display, Formatter};

impl DBConnector<Tenant> {
    pub async fn create_mycelink_account_entry(
        &self,
        tx: &mut Transaction<'_, DatabaseBackend>,
        account: MycelinkAccount,
    ) -> Result<(), MycelinkAccountEntryError> {
        if self.get_mycelink_account().await?.is_some() {
            return Err(MycelinkAccountEntryError::AccountAlreadyExists);
        }

        let query = sqlx::query(
            "INSERT INTO protocol_config_per_tenant (tenant, protocol, config) VALUES (?,?,?)",
        );
        query
            .bind(self.tenant())
            .bind(Protocol::Mycelink)
            .bind(Json(ProtocolConfig::Mycelink { account }))
            .execute(&mut **tx)
            .await?;

        Ok(())
    }

    pub async fn get_mycelink_account(&self) -> sqlx::Result<Option<MycelinkAccount>> {
        let query = sqlx::query("SELECT (config) FROM protocol_config_per_tenant WHERE protocol = 'Mycelink' AND tenant = ?")
            .bind(self.tenant());
        let res = query.fetch_optional(self.pool().await).await?;

        if let Some(row) = res {
            let account: Json<ProtocolConfig> = row.try_get("config")?;

            Ok(account.0.try_into().ok())
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug)]
pub enum MycelinkAccountEntryError {
    SqlxError { inner: sqlx::Error },
    AccountAlreadyExists,
}

impl Error for MycelinkAccountEntryError {}

impl Display for MycelinkAccountEntryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MycelinkAccountEntryError::SqlxError { inner } => inner.fmt(f),
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

#[cfg(test)]
mod tests {
    use crate::db::db_connector::DBConnector;
    use crate::mycelink::mycelink_account::MycelinkAccount;
    use crate::test::create_test_fcp_connector;
    use std::ops::Deref;

    #[tokio::test]
    async fn get_nonexistent_account() {
        let connector = DBConnector::new_testing().await.test_tenant().await;
        let mut tx = connector.begin().await.unwrap();

        assert_eq!(connector.get_mycelink_account().await.unwrap(), None);
        tx.commit().await.unwrap();
    }

    #[tokio::test]
    async fn create_and_get_account() {
        let fcp = create_test_fcp_connector("create_and_get_account").await;
        let connector = DBConnector::new_testing().await.test_tenant().await;
        let mut tx = connector.begin().await.unwrap();

        let account = MycelinkAccount::create_new("dummy account", fcp.deref())
            .await
            .unwrap();

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
