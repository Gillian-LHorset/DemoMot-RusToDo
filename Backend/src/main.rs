use axum::{routing::get, Extension, Json, Router};
use axum::http::StatusCode;
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use axum::extract::Path;
use sqlx::postgres::PgPoolOptions;

use tracing::{info, Level};
use tracing_subscriber;


#[derive(Serialize, Deserialize)]
struct TodoItem {
    todo_id: i32,
    todo_text: String,
}

#[derive(Serialize, Deserialize)]
struct CreateTodo {
    todo_text: String,
}

#[derive(Serialize, Deserialize)]
struct UpdateTodo {
    todo_text: String,
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
        //.route("/", get(hello_world_route));
        .route("/todos", get(get_todos).post(create_todo))
        .route("/todos/{id}", get(get_todo).put(update_todo))
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
    let todos = sqlx::query_as!(TodoItem, r#"SELECT todo_id, todo_text as "todo_text!" FROM public.todos ORDER BY todo_text ASC"#)
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(todos))
}

async fn get_todo(
    Extension(pool): Extension<Pool<Postgres>>,
    Path(id): Path<i32>,
) -> Result<Json<TodoItem>, StatusCode> {
    let todo = sqlx::query_as!(
    TodoItem,
    r#"SELECT todo_id, todo_text as "todo_text!" FROM public.todos WHERE todo_id = $1"#,
    id
)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(todo))
}

async fn create_todo(
    Extension(pool): Extension<Pool<Postgres>>,
    Json(new_todo): Json<CreateTodo>,
) -> Result<Json<TodoItem>, StatusCode> {
    let todo = sqlx::query_as!(
        TodoItem,
        r#"INSERT INTO public.todos (todo_text) VALUES ($1) RETURNING todo_id, todo_text"#,
        new_todo.todo_text
    )
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(todo))
}

async fn update_todo(
    Extension(pool): Extension<Pool<Postgres>>,
    Path(todo_id): Path<i32>,
    Json(updated_todo): Json<UpdateTodo>,
) -> Result<Json<TodoItem>, StatusCode> {
    let todo = sqlx::query_as!(
        TodoItem,
        r#"UPDATE todos SET todo_text = $1 WHERE todo_id = $2 RETURNING todo_id, todo_text"#,
        updated_todo.todo_text,
        todo_id
    )
        .fetch_one(&pool)
        .await;

    match todo {
        Ok(todo) => Ok(Json(todo)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}