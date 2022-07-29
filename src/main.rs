#[macro_use]
extern crate rocket;
use digital_command_control::serialport::SerialPort;
use digital_command_control::{
    power_management::{PowerManagement, PowerOn},
    Command,
};
use rocket::get;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::{openapi, openapi_get_routes, swagger_ui::*};
use std::sync::{Arc, Mutex};
use std::time::Duration;

mod cab_functions;
mod config;
use crate::config::{Cab, Config};

#[openapi(skip)]
#[get("/")]
fn index() -> Redirect {
    Redirect::to("/swagger-ui")
}

#[openapi(tag = "cabs")]
#[get("/cabs")]
pub async fn get_cabs(config: &State<Arc<Mutex<Config>>>) -> Result<Json<Vec<Cab>>, Status> {
    let config = config.try_lock().unwrap();
    let cabs = config.cabs();
    Ok(Json(cabs))
}

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build();
    let figment = rocket.figment();
    let config: Config = figment.extract().expect("config");
    let config = Arc::new(Mutex::new(config));

    let mut serialport = SerialPort::default();
    serialport.connect();
    std::thread::sleep(Duration::from_millis(10000));
    serialport
        .send(&Command::PowerManagement(PowerManagement::PowerOn(
            PowerOn::JOIN,
        )))
        .unwrap();
    let serialport = Arc::new(Mutex::new(serialport));

    println!("{:?}", config);
    rocket
        .mount(
            "/",
            openapi_get_routes![index, get_cabs, cab_functions::post_cab_function],
        )
        .mount(
            "/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .manage(config)
        .manage(serialport)
}
