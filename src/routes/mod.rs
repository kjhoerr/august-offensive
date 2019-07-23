use actix_web::{web::Json, web::Query, HttpRequest, Result};
use std::collections::HashMap;

pub mod messages;
use messages::*;

type JsonMessage<U> = Result<Json<OutgoingMsg<U>>>;

// Sends a default response message when requested an undefined resource.
pub fn not_understood(req: HttpRequest) -> JsonMessage<NotUnderstood> {
    let message = NotUnderstood {
        path: destruct_path(req.path()),
    };

    Ok(Json(message.as_outgoing()))
}

// Sends Callback message with information from HttpRequest.
pub fn callback(req: HttpRequest, query: Query<HashMap<String, String>>) -> JsonMessage<Callback> {
    let path = req.path();
    let method = req.method().as_str();

    let callback = Callback {
        path: destruct_path(path),
        request: String::from(method),
        content: query.into_inner(),
    };

    Ok(Json(callback.as_outgoing()))
}

// Takes an HttpRequest path and splits it into an array.
fn destruct_path(path: &str) -> Vec<String> {
    path.split_terminator("/")
        // first element is always blank due to link starting with "/api"
        .skip(1)
        .map(String::from)
        .collect::<Vec<String>>()
}
