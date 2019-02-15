use http::StatusCode;
use lambda_runtime::{error::HandlerError, Context};
use now_rust::{lambda, IntoResponse, Request, Response};
use std::error::Error;
use chrono::prelude::*;

fn handler(_request: Request, _c: Context) -> Result<impl IntoResponse, HandlerError> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(format!("{}", Local::now()))
        .expect("Uh, oh. Something went wrong!"))
}

fn main() -> Result<(), Box<dyn Error>> {
    Ok(lambda!(handler))
}