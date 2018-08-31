use actix_web::{HttpRequest, Json, Result, Responder};
use std::ops::Deref;

pub mod messages;
use messages::*;

pub fn not_understood(_req: &HttpRequest) -> impl Responder {
    "Hello, world!"
}

pub fn callback(req: &HttpRequest) -> Result<Json<Callback>> {
    let path = req.path();
    let query_ref = req.query();
    let request = query_ref.deref().clone();

    Ok(Json(Callback {
        path: destruct_path(path),
        request: request,
    }))
}

fn destruct_path(path: &str) -> Vec<String> {
    path.split_terminator("/")
        // first element is always blank due to link starting with "/api"
        .skip(1)
        .map(String::from)
        .collect::<Vec<String>>()
}
