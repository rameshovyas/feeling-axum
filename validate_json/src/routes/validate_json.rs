use axum::Json;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestData {
    username : String,
    password : String, 
    name : Option<String>,   
}

pub async fn validate_json(Json(data) : Json<RequestData>){
    
}