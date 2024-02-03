use core::ascii;
use std::ops::Index;

use crate::db::*;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    Json,
};
use log::{error, info, warn};
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
    match db::query_data_by_id(&state, 5881791).await {
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

#[derive(serde::Deserialize)]
// #[diesel(table_name = users)]
pub struct NewUser {
    name: String,
}

pub async fn post(
    State(_state): State<PgPool>,
    Json(new_user): Json<NewUser>,
) -> impl IntoResponse {
    // A default template or else the compiler complains
    info!("Recived body: {}", new_user.name);
    (StatusCode::OK, new_user.name)
}

pub async fn post_path(Path(id): Path<String>, State(state): State<PgPool>) -> impl IntoResponse {
    // A default template or else the compiler complains
    let cities: Vec<db::City> = Vec::new();
    let final_cities = match id.parse::<i32>() {
        Ok(id_int) => {
            info!("Recived id: {}", id);
            let final_cities = match db::query_data_by_id(&state, id_int).await {
                Ok(cities) => {
                    info!(
                        "Successfully found city with id: {}: {}",
                        id_int,
                        cities.index(0).name
                    );
                    cities
                }
                Err(err) => {
                    error!("Error querying for cities: {}", err.to_string());
                    cities
                }
            };
            info!("Final Citites: {}: {}", id_int, final_cities.index(0).name);
            final_cities
        }
        Err(err) => {
            error!("Bad Request parsing path: {}", err.to_string());
            cities
        }
    };
    // info!("Final Cities: {}: {:?}", id_int, final_cities);
    return (StatusCode::OK, Json(final_cities)).into_response();
}

pub async fn list_users(
    State(_pool): State<PgPool>,
) -> Result<Json<Vec<i32>>, (StatusCode, String)> {
    let v = vec![32];
    Ok(Json(v))
}

pub async fn lookup_city_handler(
    id: i32,
    // State(pool): State<PgPool>,
    // Extract the request body.
) -> Result<String, (StatusCode, String)> {
    // let cities = match db::query_data_by_id(&pool, id).await {
    //     Ok(cities) => (StatusCode::OK, "worked".to_string()),
    //     Err(err) => Err(err.to_string()),
    // };
    Ok(id.to_string())
}
