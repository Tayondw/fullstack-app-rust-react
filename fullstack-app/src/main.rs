use axum::{ http::{ Method, StatusCode }, response::IntoResponse, routing::get, Json, Router };
// imports Axum's HTTP and GET request and Router
// Axum is similar to Express in JS
use std::net::SocketAddr; // provides a way yo represent an IP addr and port
use tower_http::cors::{ AllowOrigin, Any, CorsLayer };

// use tokio::net::TcpListener; // NOT USED - however helps with async networking

// NEW
mod types; // importing types file - mod declares a module within the current scope, it tells Rust that a file or module exists and should be compiled
use types::Person; // using the Person struct from the types file

#[tokio::main] // marks as an async main function using Tokio runtime, which is needed to run async tasks
async fn main() {
    // let cors = CorsLayer::new().allow_origin(Any); // creates a new CORS middleware that allows requests from any origin - not safe for production

    // CORS run down: -security: prevents unwanted cross-origin requests

    // for better security of cors, use this:

    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::exact("https://your-frontend-site.com".parse().unwrap())) // allow only specific origin
        .allow_methods([Method::GET, Method::POST]) // restrict allowed methods - allows only GET and POST requests, blocks PUT, DELETE, PATCH and etc.
        .allow_headers(["Content-Type".parse().unwrap()]); // restrict allowed headers - allows only the Content-Type header, blocks others

    let app = Router::new() // creating a router instance
        .route("/", get(root)) // defines the method (GET) route for the request path and the function that will handle the request for the path
        // NEW
        .route("/people", get(get_people))
        .route("/world", get(world))
        .layer(cors); // adding this middleware to every request

    /*
      allow multiple trusted origins:
      let cors = CorsLayer::new()
            .allow_origin(AllowOrigin::list([
                  "https://your-frontend.com".parse().unwrap(),
                  "https://admin.your-frontend.com".parse().unwrap()
             ]));

      if frontend and backend are on different ports - typically during development:
      let cors = CorsLayer::new()
            .allow_origin(AllowOrigin::list([
                  "http://localhost:3000".parse().unwrap(), // Local frontend
                  "https://your-frontend.com".parse().unwrap() // Production frontend
            ]));

      */

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000)); // setting up the socket - ip addr, port, and protocol(typically TCP)
    println!("listening on {}", addr); // listening on the port - prints to the console/compiler

    axum::Server::bind(&addr) // binds the server to the specified IP and port
    .serve(app.into_make_service()) // converts the Router from line 26 into a service that Axum's server can run on
    .await // since server is async, we have to await it
    .unwrap(); // if an error starting the server, this will cause it to panic and show the error
}

async fn root() -> &'static str {
    // handler function that will be executed when a GET request is made to the / path
    "Hello, World!"
}

// NEW
async fn get_people() -> impl IntoResponse {
    let people = vec![ // creating a vector to hold each person
        Person { // populating the person struct
            name: String::from("Person A"),
            age: 36,
            favourite_food: Some(String::from("Pizza")),
        },
        Person {
            name: String::from("Person B"),
            age: 5,
            favourite_food: Some(String::from("Broccoli")),
        },
        Person {
            name: String::from("Person C"),
            age: 100,
            favourite_food: None,
        }
    ];

    (StatusCode::OK, Json(people)) // return a tuple of the status code and a json of the people vector
}

async fn world() -> &'static str { // verifying if above was how to make routes
    "hello, this is the world and the bird is the word"
}
