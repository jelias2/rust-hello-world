use crate::db::*;
use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use sqlx::{Executor, PgPool};

// #[derive(Clone)]
// pub struct Handler {
//     conn: Connection,
// }

// impl Handler {
//     pub fn new(conn: Connection) -> Handler {
//         Handler { conn: conn }
//     }
// }

pub async fn query_city(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    State(state): State<PgPool>,
    Json(_payload): Json<QueryCity>,
) -> Result<(StatusCode, Json<Vec<db::City>>), (StatusCode, Json<String>)> {
    // this will be converted into a JSON response
    // with a status code of `201 Created`
    match db::query_data_by_id(&state, "5881791".to_string()).await {
        Ok(city) => Ok((StatusCode::CREATED, Json(city))),
        Err(err) => Err((StatusCode::NOT_FOUND, Json(err.to_string()))),
    }
}
pub fn hello_from_module() {
    println!("Hello from the module!");
}

// basic handler that responds with a static string
pub async fn root() -> &'static str {
    "Hello, World!"
}

// the input to our `create_user` handler
#[derive(Deserialize)]
pub struct QueryCity {
    id: u32,
}

// the output to our `create_user` handler
#[derive(Serialize)]
pub struct User {
    id: u64,
    username: String,
}

#[derive(Serialize)]
pub struct MyData {
    message: &'static str,
}

// Handler function for the GET request
pub async fn handle_get() -> Result<Json<MyData>, StatusCode> {
    // Create an instance of the struct
    let data = MyData {
        message: "Hello, Axum!",
    };

    // Return the struct as JSON
    Ok(Json(data))
}

// we can extract the connection pool with `State`
pub async fn using_connection_pool_extractor(
    State(pool): State<PgPool>,
) -> Result<String, (StatusCode, String)> {
    sqlx::query_scalar("select 'hello world from pg'")
        .fetch_one(&pool)
        .await
        .map_err(internal_error)
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
