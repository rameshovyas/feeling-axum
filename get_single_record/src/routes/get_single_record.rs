use axum::Extension;
use serde::Serialize;
#[derive(Serialize)]
pub struct ResponseData {
id:i32,
username:string
}
pub async fn get_single_record(Path(id):Path<i32>, Extension(database): Extension<DatabaseConnection>){

}