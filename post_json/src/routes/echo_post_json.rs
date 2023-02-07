use axum::{Json};
use serde::{Deserialize,Serialize};

//struct for posted json data
#[derive(Serialize,Deserialize, Debug)]
pub struct PostedJsonData {
    data : String,
}

// struct for json response from server
#[derive(Serialize)]
pub struct JsonResponseFromServer {
    data : String,
    message_from_server: String
}

// route handler that extract incoming jsn data and creates a new json response from it.
pub async fn echo_post_json( Json(body): Json<PostedJsonData>) -> Json<JsonResponseFromServer> {
       Json(JsonResponseFromServer { data: body.data, message_from_server: "Thank you for posting JSON data".to_owned()})
}