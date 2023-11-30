use axum::{Router};
use tokio;
use cv_self;

mod routes;
mod controllers;
mod services;
mod util;
mod res;
mod error;
mod cv_play;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().nest("/", routes::compose());

    // run it with hyper on localhost:3000
    axum::Server::bind(&format!("0.0.0.0:3000").parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}