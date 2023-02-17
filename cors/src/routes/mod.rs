mod index;
mod users;
mod echo_post_string;
mod echo_post_json;
mod path_variables;
mod query_params;
mod user_agent;
mod custom_headers;

use axum::{Router, body::Body, routing::get,routing::post, http::Method};
use index::index;
use tower_http::cors::{CorsLayer, Any};
use users::users;
use echo_post_string::echo_post_string;
use echo_post_json::echo_post_json;
use path_variables::path_variables;
use query_params::query_params;
use user_agent::user_agent;
use custom_headers::custom_headers;

// public function that returns handle to all routers
pub fn create_routes() -> Router<(),Body> {
   
    //Setting up the CORS Layer
    let cors = CorsLayer::new()
        .allow_methods([Method::GET,Method::POST])
        .allow_origin(Any);
    
    Router::new()
            .route("/", get(index))
            .route("/users",get(users))
            .route("/echo_post_string", post(echo_post_string))
            .route("/echo_post_json", post(echo_post_json))
            .route("/path_variables/:id", get(path_variables))
            .route("/query_params", get(query_params))
            .route("/user_agent", get(user_agent))
            .route("/custom_headers", get(custom_headers))
            .layer(cors) // Adding the CorsLayer at the last so that it effects all the routes
}




