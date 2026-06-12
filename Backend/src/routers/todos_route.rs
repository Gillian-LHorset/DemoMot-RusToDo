use axum::{Extension, Router};
use axum::routing::get;
use sqlx::PgPool;
use crate::controllers::todo_controller;

pub fn todo_route(pool: PgPool) -> Router {
    Router::new()
        //.route("/", get(hello_world_route));
        .route("/todos", get(todo_controller::index).post(todo_controller::create))
        .route("/todo/{id}", get(todo_controller::show).put(todo_controller::update).delete(todo_controller::destroy))
        .layer(Extension(pool))
}