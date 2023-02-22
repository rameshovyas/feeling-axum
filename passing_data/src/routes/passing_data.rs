use axum::Extension;
use sea_orm::DatabaseConnection;

pub async fn passing_data(Extension(database) : Extension<DatabaseConnection>){

}