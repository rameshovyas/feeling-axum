# feeling-axum
Feeling awesome with Axum

# Table of contents
1. [Introduction](#introduction)
2. [Hello World Application](#hello_world)  
3. [Routing](#routing)
4. [Posting a String Data](#post_string)
5. [Posting JSON Data](#post_json)

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