use axum::{
    routing::get,
    Router,
};
use dotenvy::dotenv;
//use sqlx::postgres::PgPoolOptions;

use tracing::{info, Level};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {

    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    dotenv().ok();

    //let db_url = std::env::var("DATABASE_URL").expect("The DATABASE_URL environment variable is unset.");
    //let pool = PgPoolOptions::new().connect(&db_url).await?;


    println!("Connected to the database!");

    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(hello_world_route));

    let listener = tokio::net::TcpListener::bind("localhost:5000").await.unwrap();
    info!("Server is running on http://localhost:5000");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn hello_world_route() -> &'static str {
    return "Hello, world!"
}