mod passing_data;
mod create_user;
mod get_single_record;

use axum::{Router, body::Body, routing::get,routing::post, Extension};
use passing_data::passing_data;
use sea_orm::DatabaseConnection;
use create_user::create_user;
use get_single_record::get_single_record;

// public function that returns handle to all routers
pub fn create_routes(database: DatabaseConnection) -> Router<(),Body> {
    Router::new()
           .route("/passing_data", post(passing_data))
           .route("/create_user", post(create_user))
           .route("/get_single_record/:id", get(get_single_record))
           //Creating extension layer to share this database connectuion with all other routes
           .layer(Extension(database))
}

