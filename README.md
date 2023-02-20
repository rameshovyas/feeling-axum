# feeling-axum
Feeling awesome with Axum

# Table of contents
1. [Introduction](#introduction)
2. [Hello World Application](#hello_world)  
3. [Routing](#routing)
4. [Posting a String Data](#post_string)
5. [Posting JSON Data](#post_json)
6. [Dealing with Path Variables](#path_variables)
7. [Handling Query Parameters](#query_params)
8. [Extracting User Agent - Standard Headers ](#user_agent)
9. [Extracting Data from Custom Headers](#custom_headers)
10. [CORS](#cors)
11. [Sharing data among middleware layers](#shared_data)
12. [Returning HTTP Response](#http_response)
13. [Returning JSON Data](#json_response)
14. [Validating Incoming JSON Data](#validate_json)
15. [Using SeaORM to connect with postgres database](#db_seaorm)

## Introduction to axum<a name="introduction"></a>
[Axum]("https://crates.io/crates/axum") is a web application framework for Rust programming language. It is developed by the same people who developed [tokio]("https://tokio.rs/"). 

## Hello World Application <a name="hello_world"></a>
This is a very simple http server that has a single route "/", demonstrating how we can use axum to spin a simple web server. 
You can find the complete source code in **[hello_world]("https://github.com/rameshovyas/feeling-axum/tree/main/hello_world)** directory.
 
## Routing <a name="routing"></a>
This simple axum application explains how we can add different routes with different HTTP methods in our application. We have seprated the routing logic in a seprate module viz. routes that eventually imports respective route handlers from their respective code files. You can find this application in **routing** directory in this repo.

## Posting a String Data <a name="post_string"></a>
A very simple application written in axum Rust that demonstartes how we can post a string data to an api endpoint. This application creates an api endpoint that accepts a string data via HTTP POST method and echos back to the client to verify what has been extracted at api endpoint. The extraction of string in axum is very simple we just have a parameter ('body' here) of type string in the route handler, which eventually returns the same data.
```
pub async fn echo_post_string(body: String) -> String {
    body
}
```
The sample application can be found in **post_string** directory in this repo.

## Posting JSON Data <a name="post_json"></a>
Axum web application showing how to post JSON data and extract it on server fo rprocessing. It is a simple application that fetches posted JSON into a struct and then creates a new struct for JSON response from the server. We have use **serde** to deserialize and serialize json request and response. Below is the simple route handler for the same.

```
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
```

This project is available in **post_json** directory of this repo.

## Dealing with Path Variables <a name="path_variables"></a>
Simple application in axum web that demonstrates how we can extract path variables from url in an api call.
Path variables can be mapped as parameters to route handler functions. See below code that defines a route handler for **http://localhost/25**. The value **25** will be mapped to the id variable in the route handler. Using **Path** we can extract it as shown in code below:

#### Routes
```
pub fn create_routes() -> Router<(),Body> {
    Router::new()
           .route("/path_variables/:id", get(path_variables))
}

```

#### Route handler function
```
use axum::extract::Path;

pub async fn path_variables(Path(id) : Path<i32>) ->String {
    id.to_string()
}
```
The complete project can be found in **path_variables** directory in this repo.

## Handling Query Parameters <a name="query_params"></a>
As we have see in previous section regarding path variables, how we can extract path variables simply mapping it with parametrs to handler function. Query parameters are bit different to deal with as they are not part of path. Query parameters are list of key value pairs seprated by a **&** and are added to the url after **?**. To deal with query parameter we need to do bit like we have handled json data in previous section. We will be creating a struct representing our query parameters and like JSON we need to deserialize it so we will be again using **serde** . Here we have also used Serialize as we will be returning a JSON response. Below code show how we can handle this type of url : http://localhost:3000/query_params?category=programming&year=2023 . Here we have two query params (category and year). We have just echoed back the query parameters in JSON format for now, but the logic for handling query parameters is explained.

```
use axum::{extract::Query, Json};
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
pub struct QueryParams {
    category : String,
    year: i32
}
pub async fn query_params(Query(query): Query<QueryParams>) -> Json<QueryParams> {
    Json(query)
}
```
#### The JSON response we will be getting
```
{
  "category": "programming",
  "year": 2023
}
```

The complete project can be found in **query_params** directory of this repo.


## Extracting User Agent - Standard Headers <a name="user_agent"></a>
To extract standard header data for eg. user agent we will be using **TypedHeader** extractor. For this we need to add **header** feature to axum in cargo.toml file, that can be done as follows
> cargo add axum -F headers

An example route handler for extracting user agent is as follows : 

```
use axum::{TypedHeader, headers::UserAgent};
pub async fn user_agent(TypedHeader(user_agent) : TypedHeader<UserAgent>) -> String{
    user_agent.to_string()
}
```
The complete project can be found in **std_headers** directory of this repo.

## Extracting Data from Custom Headers <a name="custom_headers"></a>
Many times we need to pass custom data withing headers, in axum how to extract custom headers is very simple we just need to use **HeaderMap** for this. HeaderMap can extract custom header data.

```
use axum::http::HeaderMap;

pub async fn custom_headers(headers : HeaderMap) -> String{
   let message_value = headers.get("x-message").unwrap();
   let message = message_value.to_str().unwrap().to_owned();
   message
}

```

The complete project can be found in **custom_headers** directory of this repo.

## CORS <a name="cors"></a>
Axum does not have any middleware system of its own, where as it uses middleware from **tower** and **tower-http**. So we need to add that into our project by the following command.
> cargo add tower-http -F cors

After adding tower-http with cors feature, we need to add the following code for adding corslayer and routes.
```
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
```

The complete project can be found in **cors** directory of this repo.

## Sharing data among middleware layers <a name="shared_data"></a>
This demo project show how we can use shared data amon middleware layers in axum. The conecpt is very simple we just added a layer with the dat ausing **Extension** . Now this shared data can be extracted from any route using the same Extension. The rule is we have to keep all routes where we want to share data above the layer in the routing sequence.

The route routing sequence should be : 

```
mod index;
mod users;
mod echo_post_string;
mod echo_post_json;
mod path_variables;
mod query_params;
mod user_agent;
mod custom_headers;
mod middleware_data;

use axum::{Router, body::Body, routing::get,routing::post, http::Method, Extension};
use index::index;
use tower_http::cors::{CorsLayer, Any};
use users::users;
use echo_post_string::echo_post_string;
use echo_post_json::echo_post_json;
use path_variables::path_variables;
use query_params::query_params;
use user_agent::user_agent;
use custom_headers::custom_headers;
use middleware_data::middleware_data;

//Strcture representing shared data
#[derive(Clone)]
pub struct SharedData {
    pub data: String,
}


// public function that returns handle to all routers
pub fn create_routes() -> Router<(),Body> {
   
    //Setting up the CORS Layer
    let cors = CorsLayer::new()
        .allow_methods([Method::GET,Method::POST])
        .allow_origin(Any);

    //Instantiating shared data
    let shared_data = SharedData { data : "This is shared data".to_owned(),} ;   
    Router::new()
            .route("/", get(index))
            .route("/users",get(users))
            .route("/echo_post_string", post(echo_post_string))
            .route("/echo_post_json", post(echo_post_json))
            .route("/path_variables/:id", get(path_variables))
            .route("/query_params", get(query_params))
            .route("/user_agent", get(user_agent))
            .route("/custom_headers", get(custom_headers))
            .route("/middleware_data", get(middleware_data))
            .layer(cors) // Adding the CorsLayer at the last so that it effects all the routes

            //adding layer for shared data
            .layer(Extension(shared_data))
}

```

And the this shared data can be extracted in the route handler as below : 

```
use axum::Extension;
use super::SharedData;

pub async fn middleware_data(Extension(shared_data):Extension<SharedData>) ->String {
    shared_data.data
}
```
Find the complete project in **shared_data** directory.

## Returning HTTP Response<a name="http_response"></a>
This project demonstrates how we can return specific HTTP response from a route. For example if we want a route to respond 201 instead 200, we have to do modify our route handlers like below : 

```
use axum::http::StatusCode;
use axum::response::IntoResponse;

// Anything that implements IntoResponse can be returned from a handler
// Use `impl IntoResponse` to avoid writing the whole type
pub async fn status_codes() ->impl IntoResponse {
    (
        StatusCode::CREATED,
        "This is 201 response and not 200".into_response()
    )
}
```
The complete project is in **status_codes** directory.


## Returning JSON Data<a name="json_response"></a>
To return JSON data from a route we need to have a struct for the data and using **axum::Json** we can convert the struct data to JSON data, see the following route handler doing this.

```
use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Data {
    username: String,
    age :i32,
}

pub async fn return_json() -> Json<Data> {
    let data = Data {username:"ramesh".to_owned(), age:21,};
    Json(data)
}
```
The complete project is available in **returning_json** directory.

## Validating Incoming JSON Data<a name="validate_json"></a>
To validate incoming JSON data we need to deserialize with serde, the following route handler does an example validation.

```
use axum::Json;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestData {
    username : String,
    password : String, 
    name : Option<String>,   
}

pub async fn validate_json(Json(data) : Json<RequestData>){
    
}
```

The **Option<String>** specifies that this field is optional field.

The complete project can be found at **validate_json** directory.

## Using SeaORM to connect with postgres database<a name="db_seaorm"></a>
A simple project in axum rust to connect with postgres database and create models for the underline table so that data can be accessed inside the rust code. We have created **.env** file to store environment variable for database connection string. For handling environment variables we have used **dotenvy and dotenvy-macro** crates and for **orm** we have used **sea-orm**. The **sea-orm** itself requires **tokio** for async that is why that also has been included in the project refrence.

We have used **sea-orl-cli** to generate model files
> sea-orm-cli generate entity -o src/database

This command generated model files in **src/database** directory.

The complete project can be found at **db_seaorm** directory.