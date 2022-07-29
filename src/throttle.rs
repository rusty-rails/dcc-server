use digital_command_control::serialport::SerialPort;
use digital_command_control::{cab, Command};
use rocket::http::Status;
use rocket::post;
use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::openapi;
use schemars::JsonSchema;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Throttle {
    pub cab: u32,
    pub speed: i32,
    pub direction: usize,
}

#[openapi(tag = "throttle")]
#[post("/throttle", data = "<request>")]
pub async fn post_throttle(
    serialport: &State<Arc<Mutex<SerialPort>>>,
    request: Json<Throttle>,
) -> Result<(), Status> {
    let mut serialport = serialport.try_lock().unwrap();
    let throttle = request.into_inner();
    let command = Command::Cab(cab::Cab::Throttle(cab::Throttle {
        cab: throttle.cab,
        speed: throttle.speed,
        forward: match throttle.direction {
            0 => false,
            1 => true,
            _ => true,
        },
    }));
    serialport.send(&command).unwrap();
    println!("{:?}", throttle);
    Ok(())
}
