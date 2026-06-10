use axum::{routing::get, Extension, Json, Router};
use axum::http::StatusCode;
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

use tracing::{info, Level};
use tracing_subscriber;


#[derive(Serialize, Deserialize)]
struct TodoItem {
    todo_id: i32,
    todo_text: Option<String>, // <-- On change String en Option<String>
}

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
        // `GET /` goes to `root`
        //.route("/", get(hello_world_route));
        .route("/todos", get(get_todos))
        .layer(Extension(pool));

    let listener = tokio::net::TcpListener::bind("localhost:5000").await.unwrap();
    info!("Server is running on http://localhost:5000");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn hello_world_route() -> &'static str {
    return "Hello, world!"
}

async fn get_todos(
    Extension(pool): Extension<Pool<Postgres>>)
    -> Result<Json<Vec<TodoItem>>, StatusCode> {
    let todos = sqlx::query_as!(TodoItem, r#"SELECT todo_id, todo_text as "todo_text!" FROM public.todos ORDER BY todo_text DESC"#)
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(todos))
}