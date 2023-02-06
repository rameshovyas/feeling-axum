mod index;
mod users;
mod echo_post_string;
mod echo_post_json;
use axum::{Router, body::Body, routing::get,routing::post};
use index::index;
use users::users;
use echo_post_string::echo_post_string;
use echo_post_json::echo_post_json;

// public function that returns handle to all routers
pub fn create_routes() -> Router<(),Body> {
    Router::new()
            .route("/", get(index))
            .route("/users",get(users))
            .route("/echo_post_string", post(echo_post_string))
            .route("/echo_post_json", post(echo_post_json))
}




