#[macro_use]
extern crate rocket;

mod config;
use crate::config::Config;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build();
    let figment = rocket.figment();
    let config: Config = figment.extract().expect("config");
    println!("{:?}", config);
    rocket.mount("/", routes![index])
}
