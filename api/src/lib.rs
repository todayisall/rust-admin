// api 模块
mod common;
mod system;

// 路由模块
mod route;

use axum::{Router};
use std::net::SocketAddr;

#[tokio::main]
pub async fn run() {
    // build our application with a route
    let app = Router::new().nest("/api", route::register_router());

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 8008));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
