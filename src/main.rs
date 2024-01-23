use env_logger::Env;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use warp::{Filter, Reply};

#[tokio::main]
async fn main() {
    let debug: bool = false;
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);
    let message: String = String::from("Hello fellow Rustaceans!");
    info!("{}", message);
    if debug {
        warn!("Debug mode enabled");
    }

    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));
    // POST /employees/:rate  {"name":"Sean","rate":2}
    let promote = warp::post()
        .and(warp::path("employees"))
        .and(warp::path::param::<u32>())
        // Only accept bodies smaller than 16kb...
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        // This looks like it makes it accept a specfic body
        .map(|status_code, mut resp: ResponseObject| {
            resp.status_code = 5555;
            warp::reply::json(&resp)
        });

    let routes = hello.or(promote);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

#[derive(Debug, Serialize, Deserialize)]
struct ResponseObject {
    message: String,
    status_code: u32,
}
fn create_response_object(msg: String, status_code: u32) -> ResponseObject {
    return ResponseObject {
        message: msg,
        status_code: status_code,
    };
}

fn get_user_input() {
    let mut input = String::new();
    println!("Please enter a number 1 through 12");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(e) => {
            error!("Error reading int from input: {}", e);
            error!("Exiting program");
            return;
        }
    };

    if input > 12 || input < 1 {
        error!("Invalid input from the user");
        error!("Exiting program");
    }

    let rsp_object = create_response_object("hello world".to_string(), input);
    info!("Status Code: {}", rsp_object.status_code);
    info!("Message: {}", rsp_object.message);
}

// async fn handle_submit(data: warp::reply::Json) -> impl Reply {
//     // Handle the incoming JSON data
//     let response_message = format!("Received data: {:?}", data);

//     // Respond with a JSON message
//     let resp = create_response_object("Bad Object".to_string(), 400);
//     warp::reply::json(&resp)
// }
