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
use sqlx::PgPool;

pub async fn query_city(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    State(state): State<PgPool>,
    Json(_payload): Json<QueryCityRequest>,
) -> Result<(StatusCode, Json<Vec<db::City>>), (StatusCode, Json<String>)> {
    // this will be converted into a JSON response
    // with a status code of `201 Created`
    match db::query_data_by_id(&state, 5881791).await {
        Ok(city) => Ok((StatusCode::CREATED, Json(city))),
        Err(err) => Err((StatusCode::NOT_FOUND, Json(err.to_string()))),
    }
}

// the input to our `create_user` handler
#[derive(Deserialize)]
pub struct QueryCityRequest {
    id: u32,
}

// #[derive(Serialize)]
// pub struct QueryCityResponse {
//     message: &'static str,
//     cities: Vec<db::City>,
// }

// impl QueryCityResponse {
//     pub fn new(message: &'static str, cities: Vec<db::City>) -> QueryCityResponse {
//         QueryCityResponse {
//             message: message,
//             cities: cities,
//         }
//     }
// }

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

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

pub async fn post(
    State(state): State<PgPool>,
    Json(query_city_request): Json<QueryCityRequest>,
) -> impl IntoResponse {
    // A default template or else the compiler complains
    info!("Recived request for city ID: {}", query_city_request.id);
    let id = query_city_request.id as i32;
    let cities: Vec<db::City> = Vec::new();
    let final_cities = match db::query_data_by_id(&state, id).await {
        Ok(cities) => {
            info!(
                "Successfully found city with id: {}: {}",
                id,
                cities.index(0).name
            );
            cities
        }
        Err(err) => {
            error!("Error querying for cities: {}", err.to_string());
            cities
        }
    };
    return (StatusCode::OK, Json(final_cities)).into_response();
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
            final_cities
        }
        Err(err) => {
            error!("Bad Request parsing path: {}", err.to_string());
            cities
        }
    };
    return (StatusCode::OK, Json(final_cities));
}

pub async fn list_users(
    State(_pool): State<PgPool>,
) -> Result<Json<Vec<i32>>, (StatusCode, String)> {
    let v = vec![32];
    Ok(Json(v))
}
