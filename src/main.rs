use axum::{
    routing::{get, post},
    Router,
};
use env_logger::Env;
use log::{error, info, warn};
use std::net::SocketAddr;
mod utils;
use utils::utils::*;
mod db;
use db::db::*;
use rusqlite::{params, Connection, Result};

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

    hello_from_module();
    let conn = match Connection::open_in_memory() {
        Ok(conn) => conn,
        Err(err) => {
            error!(
                "Error creating SQLite Memory Connection: {}, Exiting...",
                err
            );
            std::process::exit(1);
        }
    };

    if let Err(err) = create_table(&conn) {
        error!("Error Creating table: {}", err);
        error!("Exiting...");
        std::process::exit(1);
    }

    if let Err(err) = read_csv_and_insert(&conn, &file_path) {
        error!("Error Reading and Inserting CSV: {}, Exiting...", err);
        std::process::exit(1);
    }

    let cities = match query_data_by_id(&conn, 5881791) {
        Ok(cities) => cities,
        Err(err) => {
            error!("Error querying by ID: {}", err);
            std::process::exit(1);
        }
    };

    for i in cities {
        info!("City Name: {} population: {}", i.name, i.population);
    }

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user))
        .route("/response", get(handle_get));

    // run our app with hyper, listening globally on port 3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
