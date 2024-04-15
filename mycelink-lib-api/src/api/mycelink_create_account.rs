use crate::api::APIConnector;
use crate::db::actions::tenant_actions::Tenant;
use crate::mycelink::mycelink_account::{CreateAccountError, MycelinkAccount};
use std::ops::Deref;

impl APIConnector<Tenant> {
    pub async fn create_mycelink_account(
        &mut self,
        display_name: impl Into<Box<str>>,
    ) -> Result<Box<str>, CreateAccountError> {
        let account = MycelinkAccount::create_new(display_name, self.fcp_connector.deref()).await?;

        let mut tx = self.db_connector.begin().await?;
        self.db_connector
            .create_mycelink_account_entry(&mut tx, &account)
            .await?;

        tx.commit().await?;

        self.load_services().await;

        Ok(account.request_ssk_key().into())
    }
}
