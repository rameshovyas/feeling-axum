use axum::{Router, body::Body, routing::get};

// public function that returns handle to all routers
pub fn create_routes() -> Router<(),Body> {
    Router::new()
            .route("/", get(index))
            .route("/users",get(users))
}


async fn index() -> String {
    "Hello World!".to_owned()
}

async fn users() -> String {
    "Get all users".to_owned()
}