#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket::serde::json::{json, Value};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

// Try make an api which takes a String integer (i32) and returns as an i32. If not i32 return -1
#[get("/parse_integer/<i32_string>")]
fn parse_integer(i32_string : String) -> Value {
    let num_result = i32_string.parse::<i32>();

    if let Err(_num_result) = num_result {
        return json!(-1);
    }

    return json!(num_result.unwrap());
}

#[get("/<name>/<age>")]
fn hello_name_age(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, hello_name_age, parse_integer])
}