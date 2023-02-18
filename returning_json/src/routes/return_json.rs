use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Data {
    username: String,
    age :i32,
}

pub async fn return_json() -> Json<Data> {
    let data = Data {username:"ramesh".to_owned(), age:21,};
    Json(data)
}