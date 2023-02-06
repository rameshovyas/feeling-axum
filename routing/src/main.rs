use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    
    //Create a route for home page
    let app = Router::new()
        .route("/",get(index));
    
    //Using Hyper to run this server
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> String {
    "Hello World!".to_owned()
}