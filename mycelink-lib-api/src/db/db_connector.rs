use sqlx::{migrate::MigrateDatabase, Sqlite, SqliteConnection, Error, SqlitePool, Pool};

use std::path::Path;

pub struct DBConnector {
    db_pool: Pool<Sqlite>,
}

impl DBConnector {

    pub async fn new(db_path: &str) -> Result<Self, sqlx::Error> {
        let db_url: &str = db_path;
        let db_pool = SqlitePool::connect(db_url).await.unwrap();
        Ok(Self{db_pool})
    }

    pub async fn initial_setup(db_path: &str) {
        let db_url: &str = db_path;
        if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
            println!("Creating database {}", db_url);
            match Sqlite::create_database(db_url).await {
                Ok(_) => println!("Create db success"),
                Err(error) => panic!("error: {}", error),
            }
        } else {
            println!("Database already exists");
        }
    }

    fn current_schema_version(&self) -> u32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn run_something() {

    }
}
