use routes::create_routes;

mod routes;

pub async fn run() {
    //Call create_routes() from the routes module for handling all routes of the application
    let app = create_routes();
    
    //Using Hyper to run this server
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

