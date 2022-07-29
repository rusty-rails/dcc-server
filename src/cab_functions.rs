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
pub struct CabFunction {
    pub cab: u32,
    pub func: i32,
    pub state: usize,
}

#[openapi(tag = "cab-functions")]
#[post("/cab-function", data = "<request>")]
pub async fn post_cab_function(
    serialport: &State<Arc<Mutex<SerialPort>>>,
    request: Json<CabFunction>,
) -> Result<(), Status> {
    let mut serialport = serialport.try_lock().unwrap();
    let cab_function = request.into_inner();
    let command = Command::Cab(cab::Cab::CabFunction(cab::CabFunction {
        cab: cab_function.cab as i32,
        func: cab_function.func,
        state: match cab_function.state {
            0 => false,
            1 => true,
            _ => true,
        },
    }));
    serialport.send(&command).unwrap();
    println!("{:?}", cab_function);
    Ok(())
}
