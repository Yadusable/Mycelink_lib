use crate::db::db_connector::{DBConnector, DatabaseBackend, TenantState};
use crate::model::tenant::Tenant;
use sqlx::{Row, Transaction};
use std::error::Error;
use std::fmt::{Display, Formatter};

impl<T: TenantState> DBConnector<T> {
    pub async fn get_tenants(
        &self,
        tx: &mut Transaction<'_, DatabaseBackend>,
    ) -> Result<Box<[Tenant]>, sqlx::Error> {
        let statement = sqlx::query("SELECT (display_name) FROM tenants;");
        let rows = statement.fetch_all(&mut **tx).await?;

        let mut res = Vec::with_capacity(rows.len());
        for row in rows.into_iter() {
            res.push(Tenant::new(
                row.try_get::<'_, Box<str>, &str>("display_name")?,
            ))
        }
        Ok(res.into())
    }

    pub async fn create_tenant(
        &self,
        tx: &mut Transaction<'_, DatabaseBackend>,
        display_name: impl Into<Box<str>>,
    ) -> Result<Tenant, sqlx::Error> {
        let display_name = display_name.into();

        let query = sqlx::query("INSERT INTO tenants VALUES (?);");
        query.bind(display_name.clone()).execute(&mut **tx).await?;
        Ok(Tenant::new(display_name))
    }

    pub async fn delete_tenant(
        &self,
        tx: &mut Transaction<'_, DatabaseBackend>,
        tenant: &Tenant,
    ) -> Result<(), DeleteTenantError> {
        let query = sqlx::query("DELETE FROM tenants WHERE display_name = ?;");
        let rows_affected = query
            .bind(tenant.display_name())
            .execute(&mut **tx)
            .await?
            .rows_affected();

        if rows_affected == 0 {
            return Err(DeleteTenantError::TenantDoesNotExist);
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum DeleteTenantError {
    SqlxError { inner: sqlx::Error },
    TenantDoesNotExist,
}

impl Error for DeleteTenantError {}

impl Display for DeleteTenantError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DeleteTenantError::SqlxError { inner } => {
                write!(f, "Failed tenant deletion due to sqlx error: {inner}")
            }
            DeleteTenantError::TenantDoesNotExist => {
                write!(f, "Tenant doesn't exist in db")
            }
        }
    }
}

impl From<sqlx::Error> for DeleteTenantError {
    fn from(value: sqlx::Error) -> Self {
        DeleteTenantError::SqlxError { inner: value }
    }
}

#[cfg(test)]
mod tests {
    use crate::db::actions::tenant_actions::DeleteTenantError;
    use crate::db::db_connector::DBConnector;
    use crate::model::tenant::Tenant;

    #[tokio::test]
    async fn get_empty_tenants() {
        let connector = DBConnector::new_testing().await;
        let mut tx = connector.begin().await.unwrap();

        let tenants = connector.get_tenants(&mut tx).await.unwrap();

        assert_eq!(*tenants, []);
        tx.commit().await.unwrap();
    }

    #[tokio::test]
    async fn create_tenant() {
        let connector = DBConnector::new_testing().await;
        let mut tx = connector.begin().await.unwrap();

        let tenant = connector
            .create_tenant(&mut tx, "Testing tenant")
            .await
            .unwrap();
        assert_eq!(tenant.display_name(), "Testing tenant");

        let got_tenants = connector.get_tenants(&mut tx).await.unwrap();
        assert_eq!(*got_tenants, [tenant]);

        tx.commit().await.unwrap();
    }

    #[tokio::test]
    async fn create_multiple_tenants() {
        let connector = DBConnector::new_testing().await;
        let mut tx = connector.begin().await.unwrap();
        let base = "Test tenant";

        let mut expected_tenants = vec![];

        for i in 0..5 {
            expected_tenants.push(
                connector
                    .create_tenant(&mut tx, format!("{base} {i}"))
                    .await
                    .unwrap(),
            );
        }

        let got_tenants = connector.get_tenants(&mut tx).await.unwrap();

        assert_eq!(*got_tenants, *expected_tenants);
        tx.commit().await.unwrap();
    }

    #[tokio::test]
    async fn create_delete_tenant() {
        let connector = DBConnector::new_testing().await;
        let mut tx = connector.begin().await.unwrap();

        let t = connector
            .create_tenant(&mut tx, "Testing tenant")
            .await
            .unwrap();
        connector.delete_tenant(&mut tx, &t).await.unwrap();

        assert_eq!(*connector.get_tenants(&mut tx).await.unwrap(), []);
        tx.commit().await.unwrap();
    }

    #[tokio::test]
    async fn delete_non_existent_tenant() {
        let connector = DBConnector::new_testing().await;
        let mut tx = connector.begin().await.unwrap();

        let err = connector
            .delete_tenant(&mut tx, &Tenant::new("Doesn't exist"))
            .await
            .unwrap_err();

        let expected_err = DeleteTenantError::TenantDoesNotExist;

        assert_eq!(format!("{err:?}"), format!("{expected_err:?}"));
        tx.commit().await.unwrap();
    }
}
