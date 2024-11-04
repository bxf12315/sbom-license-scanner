use sea_orm::{Database, DatabaseConnection, DbErr};

async fn init_db(isTest: bool, db_path: String) -> Result<DatabaseConnection, DbErr> {
    if (!isTest) {
        let db_connection_string = "sqlite://";
        let db_connection_string = format!("{} {}", db_connection_string, db_path.as_str());
        let db = Database::connect(db_connection_string).await?;
        Ok(db)
    } else {
        let db = Database::connect("sqlite::memory:").await?;
        Ok(db)
    }
}
