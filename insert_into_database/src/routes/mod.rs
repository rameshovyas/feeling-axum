mod passing_data;
mod create_user;
use axum::{Router, body::Body, routing::get,routing::post, Extension};
use passing_data::passing_data;
use sea_orm::DatabaseConnection;
use create_user::create_user;

// public function that returns handle to all routers
pub fn create_routes(database: DatabaseConnection) -> Router<(),Body> {
    Router::new()
           .route("/passing_data", post(passing_data))
           .route("/create_user", post(create_user))
           //Creating extension layer to share this database connectuion with all other routes
           .layer(Extension(database))
}

