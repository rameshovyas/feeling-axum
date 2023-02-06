mod index;
mod users;
use axum::{Router, body::Body, routing::get};
use index::index;
use users::users;

// public function that returns handle to all routers
pub fn create_routes() -> Router<(),Body> {
    Router::new()
            .route("/", get(index))
            .route("/users",get(users))
}




