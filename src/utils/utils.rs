use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

pub fn hello_from_module() {
    println!("Hello from the module!");
}

// basic handler that responds with a static string
pub async fn root() -> &'static str {
    "Hello, World!"
}

// the input to our `create_user` handler
#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
pub struct User {
    id: u64,
    username: String,
}

pub async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// pub struct ResponseObject {
//     message: String,
//     status_code: u32,
// }

// pub async fn create_response_object(// this argument tells axum to parse the request body
//     // as JSON into a `CreateUser` type
//     // Json(payload): Json<CreateUser>,
// ) -> (StatusCode, Json<ResponseObject>) {
//     // insert your application logic here
//     let resp = ResponseObject {
//         message: "Hello dumbass".to_string(),
//         status_code: 42,
//     };

//     // this will be converted into a JSON response
//     // with a status code of `201 Created`
//     (StatusCode::ACCEPTED, Json(resp))
// }

// Handler function for the GET request
// pub async fn handle_get() -> Result<Json<&'static ResponseObject>, StatusCode> {
//     let resp = ResponseObject {
//         message: "This is a GET request handler".to_string(),
//         status_code: 200,
//     };
//     Ok(Json(resp))ub
// }

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
    // Err(StatusCode::ACCEPTED.into())
}
