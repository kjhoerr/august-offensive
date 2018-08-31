use actix_web::{HttpRequest, Json, Result};
use std::ops::Deref;

pub mod messages;
use messages::*;

type JsonMessage<U> = Result<Json<OutgoingMsg<U>>>;

// Sends a default response message when requested an undefined resource.
pub fn not_understood(req: &HttpRequest) -> JsonMessage<NotUnderstood> {
    let message = NotUnderstood {
        path: destruct_path(req.path()),
    };

    Ok(Json(message.as_outgoing()))
}

// Sends Callback message with information from HttpRequest.
pub fn callback(req: &HttpRequest) -> JsonMessage<Callback> {
    let path = req.path();
    let method = req.method().as_str();
    let query_ref = req.query();
    let request = query_ref.deref().clone();

    let callback = Callback {
        path: destruct_path(path),
        request: String::from(method),
        content: request,
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
