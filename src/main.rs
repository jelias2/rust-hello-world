mod db;
mod prometheus;
mod utils;

use axum::{
    middleware::{self},
    routing::get,
    routing::post,
    Router,
};
use db::db::*;
use env_logger::Env;
use log::{error, info, warn};
use prometheus::prometheus::*;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::time::Duration;

#[tokio::main]

async fn main() {
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "info")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(env);

    let file_path =
        std::env::var("RS_FILE_PATH").unwrap_or("./data/cities_usa_canada.csv".to_string());
    let debug = std::env::var("RS_DEBUG")
        .unwrap_or("FALSE".to_string())
        .to_lowercase()
        == "true";
    info!("File Path: {}", file_path);
    if debug {
        warn!("Debug mode enabled");
    }

    let db_connection = "postgres://postgres:postgres@127.0.0.1:5432/postgres";
    let db_connection_str =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| db_connection.to_string());
    info!("Attempting to connect to: {}", db_connection_str);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    if let Err(err) = create_table(&pool).await {
        error!("Error Creating table: {}", err);
        error!("Exiting...");
        std::process::exit(1);
    }

    if let Err(err) = read_csv_and_insert(&pool, &file_path).await {
        error!("Error Reading and Inserting CSV: {}, Exiting...", err);
        std::process::exit(1);
    }

    let cities = match query_data_by_id(&pool, 5881791).await {
        Ok(cities) => cities,
        Err(err) => {
            error!("Error querying by ID: {}", err);
            std::process::exit(1);
        }
    };

    for i in cities {
        info!("City Name: {} population: {}", i.name, i.population);
    }

    // run our app with hyper, listening globally on port 3000
    // The `/metrics` endpoint should not be publicly available. If behind a reverse proxy, this
    // can be achieved by rejecting requests to `/metrics`. In this example, a second server is
    // started on another port to expose `/metrics`.
    let (_main_server, _metrics_server) =
        tokio::join!(start_main_server(pool), start_metrics_server());
}

fn main_app(pool: PgPool) -> Router {
    // build our application with a route
    Router::new()
        .route("/user/list", get(utils::utils::list_users))
        .route("/health", get(utils::utils::handle_get))
        .route("/post", post(utils::utils::post))
        .route("/post/:id", post(utils::utils::post_path))
        .route_layer(middleware::from_fn(track_metrics))
        .with_state(pool)
}

async fn start_main_server(pool: PgPool) {
    let app = main_app(pool);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
