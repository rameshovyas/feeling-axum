use axum::{extract::Query, Json};
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
pub struct QueryParams {
    category : String,
    year: i32
}
pub async fn query_params(Query(query): Query<QueryParams>) -> Json<QueryParams> {
    Json(query)
}