use crate::db::db_connector::{DBConnector, TenantState};
use futures::{Stream, StreamExt};
use sqlx::database::{HasArguments, HasValueRef};
use sqlx::encode::IsNull;
use sqlx::error::BoxDynError;
use sqlx::sqlite::{SqliteArgumentValue, SqliteTypeInfo};
use sqlx::{Decode, Encode, Row, Sqlite, Type};
use std::borrow::Cow;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Tenant {
    display_name: Box<str>,
}

impl Tenant {
    pub(crate) fn new(display_name: impl Into<Box<str>>) -> Tenant {
        Self {
            display_name: display_name.into(),
        }
    }

    pub fn display_name(&self) -> &str {
        &self.display_name
    }
}

impl Encode<'_, Sqlite> for &Tenant {
    fn encode_by_ref(&self, buf: &mut <Sqlite as HasArguments<'_>>::ArgumentBuffer) -> IsNull {
        buf.push(SqliteArgumentValue::Text(Cow::Owned(
            self.display_name().into(),
        )));
        IsNull::No
    }
}
impl Type<Sqlite> for Tenant {
    fn type_info() -> SqliteTypeInfo {
        <&str as Type<Sqlite>>::type_info()
    }
}
impl Decode<'_, Sqlite> for Tenant {
    fn decode(value: <Sqlite as HasValueRef<'_>>::ValueRef) -> Result<Self, BoxDynError> {
        let value = <&str as Decode<Sqlite>>::decode(value)?;
        Ok(Tenant {
            display_name: value.into(),
        })
    }
}
impl<T: Into<Box<str>>> From<T> for Tenant {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T: TenantState> DBConnector<T> {
    pub async fn get_tenants(&self) -> impl Stream<Item = sqlx::Result<Tenant>> + '_ {
        let statement = sqlx::query("SELECT (display_name) FROM tenants;");
        let rows = statement.fetch(self.pool().await);

        rows.map(|e| e.map(|e| e.get("display_name")))
    }

    pub async fn has_tenant(&self, tenant: &str) -> sqlx::Result<bool> {
        let query = sqlx::query("SELECT COUNT(*) as count FROM tenants WHERE display_name = ?;")
            .bind(tenant);

        let res = query.fetch_one(self.pool().await).await?;

        Ok(res.get::<i32, &str>("count") > 0)
    }

    pub async fn create_tenant(
        &self,
        display_name: impl Into<Box<str>>,
    ) -> Result<Tenant, sqlx::Error> {
        let display_name = display_name.into();

        let query = sqlx::query("INSERT INTO tenants VALUES (?);");
        query
            .bind(display_name.clone())
            .execute(self.pool().await)
            .await?;
        Ok(Tenant::new(display_name))
    }

    pub async fn delete_tenant(&self, tenant: &Tenant) -> Result<(), DeleteTenantError> {
        let query = sqlx::query("DELETE FROM tenants WHERE display_name = ?;");
        let rows_affected = query
            .bind(tenant.display_name())
            .execute(self.pool().await)
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
    use crate::db::actions::tenant_actions::{DeleteTenantError, Tenant};
    use crate::db::db_connector::DBConnector;
    use futures::StreamExt;

    #[tokio::test]
    async fn get_empty_tenants() {
        let connector = DBConnector::new_testing().await;

        let mut tenants = connector.get_tenants().await;

        assert!(tenants.next().await.is_none());
    }

    #[tokio::test]
    async fn create_tenant() {
        let connector = DBConnector::new_testing().await;

        let tenant = connector.create_tenant("Testing tenant").await.unwrap();
        assert_eq!(tenant.display_name(), "Testing tenant");

        let got_tenants: Vec<Tenant> = connector
            .get_tenants()
            .await
            .map(|e| e.unwrap())
            .collect()
            .await;
        assert_eq!(*got_tenants, [tenant]);
    }

    #[tokio::test]
    async fn create_multiple_tenants() {
        let connector = DBConnector::new_testing().await;
        let base = "Test tenant";

        let mut expected_tenants = vec![];

        for i in 0..5 {
            expected_tenants.push(
                connector
                    .create_tenant(format!("{base} {i}"))
                    .await
                    .unwrap(),
            );
        }

        let got_tenants: Vec<Tenant> = connector
            .get_tenants()
            .await
            .map(|e| e.unwrap())
            .collect()
            .await;

        assert_eq!(*got_tenants, *expected_tenants);
    }

    #[tokio::test]
    async fn create_delete_tenant() {
        let connector = DBConnector::new_testing().await;

        let t = connector.create_tenant("Testing tenant").await.unwrap();
        connector.delete_tenant(&t).await.unwrap();

        assert_eq!(
            *connector
                .get_tenants()
                .await
                .map(|e| e.unwrap())
                .collect::<Vec<Tenant>>()
                .await,
            []
        );
    }

    #[tokio::test]
    async fn delete_non_existent_tenant() {
        let connector = DBConnector::new_testing().await;

        let err = connector
            .delete_tenant(&Tenant::new("Doesn't exist"))
            .await
            .unwrap_err();

        let expected_err = DeleteTenantError::TenantDoesNotExist;

        assert_eq!(format!("{err:?}"), format!("{expected_err:?}"));
    }
}
