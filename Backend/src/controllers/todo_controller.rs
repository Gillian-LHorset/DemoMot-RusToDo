use axum::{Extension, Json};
use axum::extract::Path;
use axum::http::StatusCode;
use serde_json::Number;
use sqlx::{Pool, Postgres};

use crate::models::todo_model::{TodoItem, CreateTodo, UpdateTodo};

pub async fn index(
    Extension(pool): Extension<Pool<Postgres>>,
    user_id: i32)
    -> Result<Json<Vec<TodoItem>>, StatusCode> {
    let todos = sqlx::query_as!(TodoItem, r#"SELECT todo_id, todo_text as "todo_text!"
                                                            FROM public.todos
                                                            WHERE user_fk = $1
                                                            ORDER BY todo_text ASC"#,
        user_id)

        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(todos))
}

pub async fn show(
    Extension(pool): Extension<Pool<Postgres>>,
    Path(todo_id): Path<i32>,
) -> Result<Json<TodoItem>, StatusCode> {
    let todo = sqlx::query_as!(
    TodoItem,
    r#"SELECT todo_id, todo_text as "todo_text!" FROM public.todos WHERE todo_id = $1"#,
    todo_id
)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(todo))
}

pub async fn create(
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

pub async fn update(
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

pub async fn destroy(
    Extension(pool): Extension<Pool<Postgres>>,
    Path(todo_id): Path<i32>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let result = sqlx::query!("DELETE FROM todos WHERE todo_id = $1", todo_id)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => Ok(Json(serde_json::json! ({
            "message": "Todo deleted successfully"
        }))),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}