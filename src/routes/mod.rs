use actix_web::{HttpRequest, Json, Result, Responder};
use std::ops::Deref;

pub mod messages;
use messages::*;

pub fn not_understood(_req: &HttpRequest) -> impl Responder {
    "Hello, world!"
}

pub fn callback(req: &HttpRequest) -> Result<Json<Callback>> {
    let path = String::from(req.path());
    let request = (*(req.query().deref())).clone();

    Ok(Json(Callback {
        path: destruct_path(path),
        request: request,
    }))
}

fn destruct_path(path: String) -> Vec<String> {
    path.split_terminator("/")
        .skip(1)
        .map(|s| String::from(s))
        .collect::<Vec<String>>()
}