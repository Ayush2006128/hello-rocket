#[macro_use] extern crate rocket;

use rocket::form::FromForm;

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

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, calculate])
}