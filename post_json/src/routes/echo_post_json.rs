use axum::{Json};
use serde::{Deserialize,Serialize};

#[derive(Serialize,Deserialize, Debug)]
pub struct PostedJsonData {
    data : String,
}

#[derive(Serialize)]
pub struct JsonResponseFromServer {
    data : String,
    message_from_server: String
}

pub async fn echo_post_json( Json(body): Json<PostedJsonData>) -> Json<JsonResponseFromServer> {
       Json(JsonResponseFromServer { data: body.data, message_from_server: "Thank you for posting JSON data".to_owned()})
}