use crate::db::db_connector::DatabaseBackend;
use sqlx::{Pool, Transaction};

pub async fn update_to_newest_version(
    current_version: u32,
    pool: &Pool<DatabaseBackend>,
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    update_to_v1(current_version, &mut tx).await?;

    tx.commit().await?;
    Ok(())
}

async fn update_to_v1(
    current_version: u32,
    tx: &mut Transaction<'_, DatabaseBackend>,
) -> Result<(), sqlx::Error> {
    match current_version {
        1.. => Ok(()),
        0 => {
            log::info!("Updating db schema to v1");
            let query = sqlx::query(include_str!("db_schema_v1.sql"));
            query.execute(&mut **tx).await?;

            let query = sqlx::query("INSERT INTO database_metadata (schema_version) VALUES (1)");
            query.execute(&mut **tx).await?;

            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::db::db_connector::DBConnector;
    use crate::db::schema_updater::update_to_v1;
    use sqlx::sqlite::SqlitePoolOptions;

    #[tokio::test]
    async fn test_update_v1() {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .idle_timeout(None)
            .max_lifetime(None)
            .connect("sqlite://")
            .await
            .unwrap();

        let mut tx = pool.begin().await.unwrap();
        update_to_v1(0, &mut tx).await.unwrap();
        tx.commit().await.unwrap();

        assert_eq!(DBConnector::current_schema_version(&pool).await.unwrap(), 1);
    }
}
