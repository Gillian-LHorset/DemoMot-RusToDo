use axum::{routing::get, Extension, Router};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{Postgres};
use sqlx::postgres::PgPoolOptions;

use tracing::{info, Level};
use tracing_subscriber;

// import controllers
mod controllers;
use controllers::*;

mod models;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {

    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("The DATABASE_URL environment variable is unset.");
    let pool = PgPoolOptions::new().connect(&db_url).await?;


    println!("Connected to the database!");

    let app = Router::new()
        .nest("/api", Router::new()
            //.route("/", get(hello_world_route));
            .route("/todos", get(todo_controller::get_todos).post(todo_controller::create_todo))
            .route("/todo/{id}", get(todo_controller::get_todo).put(todo_controller::update_todo).delete(todo_controller::delete_todo))
            .layer(Extension(pool))
        );

    let listener = tokio::net::TcpListener::bind("localhost:5000").await.unwrap();
    info!("Server is running on http://localhost:5000");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn hello_world_route() -> &'static str {
    return "Hello, world!"
}

