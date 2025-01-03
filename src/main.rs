use std::println as printf;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use hyper::Server;
use hyper::service::service_fn;
use hyper::{Body, Request, Response, StatusCode};
use tokio::async::io::AsyncReadExt;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::prelude::*;
use tracing_subscriber::util::SubscriberInitExt;
use mysql_async::Pool;
use mysql_async::prelude::Queryable;
use hyper::http::Request;
use hyper::service::Service;
use hyper::Body;
use std::convert::Infallible;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::runtime::Runtime;
use chrono::prelude::*;
use std::time::Duration;
use std::thread::sleep;
use std::sync::Arc;
use std::sync::Mutex;
use std::collections::HashMap;
include_str!("../frontend/index.html");


async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Error> {
    // Handle the request
    Ok(Response::new(Body::from(r#"Welcome to a Rust web server"#)))
}

fn main() {
    printf!("Welcome to a Rust web server!");
/*
let pool = Pool::new("mysql://homestead:secret@localhost:3306/homestead").unwrap();
let mut conn = pool.get_conn().unwrap();

// Query the database
let results = conn.query_map("SELECT * FROM users", |(id, name)| User { id, name }).unwrap();
for user in results {
    println!("Found user: {} with id {}", user.name, user.id);
}
*/
    let make_service = || {
        let pool = pool.clone();
        service_fn(move |req| handle_request(req, pool.clone()))
    };

    let addr = ([127, 0, 0, 1], 3300).into();

    let service = make_service
        .and_then(|r| r)
        .serve(addr)
        .with(TraceLayer::new_for_http())
        .with(CorsLayer::new().allow_origin("http://localhost:3300"));

    info!("Starting server at http://localhost:3300");

    Server::bind(&addr)
        .serve(service)
        .await
        .unwrap();
    // Read the contents of the index.html file
    let mut file = File::open("index.html").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Initialize the database connection pool
    let db_url = "mysql://homestead:secret@127.0.0.1:3306/homestead";
    let pool = mysql_async::Pool::new(db_url);
}
