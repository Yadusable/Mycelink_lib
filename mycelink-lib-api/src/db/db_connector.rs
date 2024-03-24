use crate::db::schema_updater::update_to_newest_version;
use crate::model::tenant::Tenant;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{Executor, Pool, Row, Sqlite, SqlitePool, Transaction};

#[cfg(test)]
use sqlx::sqlite::SqlitePoolOptions;

pub type DatabaseBackend = Sqlite;

pub struct DBConnector<T: TenantState> {
    pool: Pool<Sqlite>,
    tenant: T,
}

pub trait TenantState {}

pub type NoTenant = ();

impl TenantState for NoTenant {}

impl TenantState for Tenant {}

impl DBConnector<NoTenant> {
    pub async fn new(db_path: &str) -> Result<DBConnector<NoTenant>, sqlx::Error> {
        let pool = Self::connect(db_path).await?;

        let current_schema_version = Self::current_schema_version(&pool).await?;
        update_to_newest_version(current_schema_version, &pool).await?;

        Ok(DBConnector { pool, tenant: () })
    }

    async fn connect(uri: &str) -> Result<Pool<DatabaseBackend>, sqlx::Error> {
        let connect_options = SqliteConnectOptions::new()
            .create_if_missing(true)
            .filename(uri);
        SqlitePool::connect_with(connect_options).await
    }

    async fn current_schema_version(pool: &Pool<DatabaseBackend>) -> Result<u32, sqlx::Error> {
        let mut conn = pool.acquire().await?;

        let res = conn
            .fetch_one(r"SELECT (schema_version) from database_metadata")
            .await;

        Ok(res.map(|row| row.get::<u32, _>(0)).unwrap_or(0))
    }
}

impl<T: TenantState> DBConnector<T> {
    pub async fn begin(&self) -> Result<Transaction<DatabaseBackend>, sqlx::Error> {
        self.pool.begin().await
    }
}

impl DBConnector<Tenant> {
    pub fn tenant(&self) -> &Tenant {
        &self.tenant
    }
}
#[cfg(test)]
impl DBConnector<NoTenant> {
    pub async fn new_testing() -> DBConnector<NoTenant> {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .idle_timeout(None)
            .max_lifetime(None)
            .connect("sqlite://")
            .await
            .unwrap();

        let current_schema_version = Self::current_schema_version(&pool).await.unwrap();
        update_to_newest_version(current_schema_version, &pool)
            .await
            .unwrap();

        DBConnector { pool, tenant: () }
    }
}

#[cfg(test)]
impl DBConnector<NoTenant> {
    pub async fn test_tenant(self) -> DBConnector<Tenant> {
        let mut tx = self.begin().await.unwrap();
        let tenant = self.create_tenant(&mut tx, "Test Tenant").await.unwrap();
        tx.commit().await.unwrap();

        DBConnector {
            pool: self.pool,
            tenant,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::db::db_connector::DBConnector;
    use std::env::temp_dir;
    use std::path::PathBuf;

    use tokio::fs;

    async fn prepare_clean_path(test_name: &str) -> PathBuf {
        let mut path = temp_dir();
        path.push(format!("mycelink-{test_name}.sqlite"));

        if path.exists() {
            fs::remove_file(path.as_path()).await.unwrap();
        }
        assert!(!path.exists());
        path
    }

    #[tokio::test]
    async fn test_connect_creates_database_file() {
        let path = prepare_clean_path("test_connect_creates_database_file").await;

        DBConnector::connect(path.to_str().unwrap()).await.unwrap();

        assert!(path.exists())
    }

    #[tokio::test]
    async fn test_current_schema_version_v0() {
        let path = prepare_clean_path("test_current_schema_version_v0").await;
        let pool = DBConnector::connect(path.to_str().unwrap()).await.unwrap();

        assert_eq!(DBConnector::current_schema_version(&pool).await.unwrap(), 0);
    }

    #[tokio::test]
    async fn test_new_initializes_db() {
        let path = prepare_clean_path("test_new_initializes_db").await;
        let connector = DBConnector::new(path.to_str().unwrap()).await.unwrap();

        assert!(
            DBConnector::current_schema_version(&connector.pool)
                .await
                .unwrap()
                > 0
        );
    }
}
