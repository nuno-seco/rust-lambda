// use std::error::Error;

use lambda_runtime::{error::HandlerError, lambda, Context};
use log::{self, error, LevelFilter};
use serde_derive::{Deserialize, Serialize};
use simple_error::bail;
use simple_logger::SimpleLogger;
use std::fs;

#[derive(Deserialize)]
struct CustomEvent {
    #[serde(rename = "firstName")]
    first_name: String,
}

#[derive(Serialize)]
struct CustomOutput {
    message: String,
}

fn main() -> () {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    lambda!(my_handler);
}

fn my_handler(e: CustomEvent, c: Context) -> Result<CustomOutput, HandlerError> {
    if e.first_name == "" {
        error!("Empty first name in request {}", c.aws_request_id);
        bail!("Empty first name");
    }

    let contents = fs::read_to_string("/cenas").expect("Something went wrong reading the file");

    Ok(CustomOutput {
        message: format!("Hello, {}! {}!", e.first_name, contents),
    })
}
