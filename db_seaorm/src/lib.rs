use sea_orm::Database;

pub async fn run(database_url:&str) {
    let database = Database::connect(database_url).await;
}