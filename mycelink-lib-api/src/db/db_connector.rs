use crate::db::schema_updater::update_to_newest_version;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{Executor, Pool, Row, Sqlite, SqlitePool};

pub type DatabaseBackend = Sqlite;

pub struct DBConnector {
    db_pool: Pool<Sqlite>,
}

impl DBConnector {
    pub async fn new(db_path: &str) -> Result<Self, sqlx::Error> {
        let db_pool = Self::connect(db_path).await?;

        let current_schema_version = Self::current_schema_version(&db_pool).await?;
        update_to_newest_version(current_schema_version, &db_pool).await?;

        Ok(Self { db_pool })
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

    pub fn has_account(&self, public_key: &str) {
        todo!()
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
            DBConnector::current_schema_version(&connector.db_pool)
                .await
                .unwrap()
                > 0
        );
    }
}
