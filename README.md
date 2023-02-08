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