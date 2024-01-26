use axum::{
    routing::{get, post},
    Router,
};
use env_logger::Env;
use log::{error, info, warn};
use std::net::SocketAddr;
mod utils;
use utils::utils::*;

#[tokio::main]

async fn main() {
    let debug: bool = false;
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "info")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);
    let message: String = String::from("Hello fellow Rustaceans!");
    info!("{}", message);
    if debug {
        warn!("Debug mode enabled");
    }

    hello_from_module();

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
