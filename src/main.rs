use env_logger::Env;
use log::{error, info, warn};

fn main() {
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
    // warn!("Hello from here");

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
}
