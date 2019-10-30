use actix_web::{http::StatusCode, Error, HttpRequest, HttpResponse, Responder};
use serde::Serialize;

pub struct FormatMsg<T> {
    pub message: T,
    pub code: StatusCode,
}

impl<T> FormatMsg<T> {
    /// Deconstruct to an inner value
    pub fn into_inner(self) -> T {
        self.message
    }

    pub fn ok(message: T) -> Self {
        FormatMsg {
            message: message,
            code: StatusCode::OK,
        }
    }
}

impl<T: Serialize> Responder for FormatMsg<T> {
    type Error = Error;
    type Future = Result<HttpResponse, Error>;

    fn respond_to(self, _: &HttpRequest) -> Self::Future {
        let body = match serde_json::to_string(&self.message) {
            Ok(body) => body,
            Err(e) => return Err(e.into()),
        };

        Ok(HttpResponse::build(self.code)
            .content_type("application/json")
            .body(body))
    }
}