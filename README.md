# feeling-axum
Feeling awesome with Axum

# Table of contents
1. [Introduction](#introduction)
2. [Hello World Application](#hello_world)  
3. [Routing](#routing)


## Introduction to axum<a name="introduction"></a>
[Axum]("https://crates.io/crates/axum") is a web application framework for Rust programming language. It is developed by the same people who developed [tokio]("https://tokio.rs/"). 

## Hello World Application <a name="hello_world"></a>
This is a very simple http server that has a single route "/", demonstrating how we can use axum to spin a simple web server. 
You can find the complete source code in **[hello_world]("https://github.com/rameshovyas/feeling-axum/tree/main/hello_world)** directory.
 
## Routing <a name="routing"></a>
This simple axum application explains how we can add different routes with different HTTP methods in our application. We have seprated the routing logic in a seprate module viz. routes that eventually imports respective route handlers from their respective code files. You can find this application in **routing** folder in this repo.