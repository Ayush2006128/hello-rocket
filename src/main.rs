#[macro_use] extern crate rocket;

use rocket::{form::FromForm, http::Method};
use rocket_cors::{AllowedOrigins, CorsOptions};

/// A struct for calculator query
/// It has three fields:
///     1. `method` It represents the mathematical methods to be performed.
///     2. `a` It represents LHS.
///     3. `b` It represents RHS.
#[derive(FromForm)]
struct CalcQuery {
    method: String,
    a: f32,
    b: f32,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
/// A simple route for performing arithmetic operations.
/// Args:
///     query as type of CalcQuery
/// Panics:
///     The code does not stop if invalid methods passed or tried to divide by 0.
///     It simply returns a warning message.
#[get("/calculate?<query..>")]
fn calculate(query: CalcQuery) -> String {
    match query.method.as_str() {
        "add" => (query.a + query.b).to_string(),
        "sub" => (query.a - query.b).to_string(),
        "mul" => (query.a * query.b).to_string(),
        "div" => {
            if query.b as i32 == 0 {
                "ZERO DIVISION ERROR!!!".into()
            } else {
                (query.a / query.b).to_string()
            }
        }
        _ => "UNKNOWN METHOD!".into(),
    }
}

// warning the logic shouldn't be used in production!
#[shuttle_runtime::main]
async fn rocket() -> shuttle_rocket::ShuttleRocket {
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);

    let rocket = rocket::build()
        .attach(cors.to_cors().unwrap())
        .mount("/", routes![index, calculate]);

    Ok(rocket.into())
}
