use axum::Router;
use tokio;

mod controllers;
mod cv_play;
mod error;
mod res;
mod routes;
mod services;
mod util;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().nest("/", routes::compose());

    // run it with hyper on localhost:3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
