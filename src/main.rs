// use axum::{routing::get, routing::post, Router};
use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use env_logger::Env;
use log::{error, info, warn};
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use tokio::net::TcpListener;
mod db;
mod utils;
use db::db::*;
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

    // let handler = utils::utils::Handler::new(conn);

    // build our application with a route
    let app = Router::new()
        // let app = Router::new()
        .route("/user/list", get(utils::utils::list_users))
        // .route("/user/create", post(utils::utils::create_user))
        .route("/post", post(utils::utils::post))
        // .with_state(pool);
        // `GET /` goes to `root`
        // .route("/city", post(utils::utils::create_user))
        // .route("/", get(utils::utils::using_connection_pool_extractor))
        .with_state(pool);
    // .route("/response", get(handle_get)

    // run our app with hyper, listening globally on port 3000
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    // tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
