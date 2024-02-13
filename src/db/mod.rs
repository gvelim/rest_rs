use sea_orm::*;

use crate::AppConfig;

pub async fn connect(config: &AppConfig) -> Result<DatabaseConnection,DbErr> {
    let path = std::env::current_dir().unwrap();
    let mut ops = ConnectOptions::new(
        format!("sqlite://{}/{}.sqlite?mode=rwc",path.display(),config.db_database)
    );
    ops.sqlx_logging(false);
    Database::connect(ops).await
}