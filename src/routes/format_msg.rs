use actix_web::{body::BoxBody, http::StatusCode, HttpRequest, HttpResponse, Responder};
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
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<BoxBody> {
        let body = match serde_json::to_string(&self.message) {
            Ok(serstr) => serstr,
            Err(e) => return HttpResponse::from_error(e),
        };

        HttpResponse::build(self.code)
            .content_type("application/json")
            .body(body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::routes::*;
    use crate::routes::tests::*;
    use serde::ser::{Error, Serializer};

    #[test]
    fn test_into_inner() {
        // Arrange
        let msg = NotUnderstood {path: vec![]};
        let msg_ref = msg.clone();
        let formatted = FormatMsg {
            message: msg.as_outgoing(),
            code: StatusCode::OK,
        };

        // Act
        let result = formatted.into_inner();

        // Assert
        assert_eq!(result.result_type, "NOT_UNDERSTOOD");
        assert_eq!(result.content, msg_ref);
    }

    #[test]
    fn test_ok() {
        // Arrange
        let msg = NotUnderstood {path: vec![]};
        let msg_ref = msg.clone();

        // Act
        let result = FormatMsg::ok(msg);

        // Assert
        assert_eq!(result.message, msg_ref);
        assert_eq!(result.code, StatusCode::OK);
    }

    #[test]
    fn test_responder() {
        // Arrange
        let msg = NotUnderstood {path: vec![]};
        let msg_ref = msg.clone();
        let formatted = FormatMsg {
            message: msg,
            code: StatusCode::NOT_FOUND,
        };
        let request = gen_request("/api/404", None);

        // Act
        let result = formatted.respond_to(&request);

        // Assert
        assert_eq!(result.status(), StatusCode::NOT_FOUND);
        assert_eq!(result.headers().get("content-type").unwrap(), "application/json");

        let resp = get_message::<NotUnderstood>(result);
        assert!(resp.is_ok());
        let content = resp.unwrap();
        assert_eq!(content, msg_ref);
    }

    struct InvalidMessage {}

    impl Serialize for InvalidMessage {
        fn serialize<S>(&self, _: S) -> Result<S::Ok, S::Error> where S: Serializer {
            Err(Error::custom("oops".to_string()))
        }
    }

    #[test]
    fn test_responder_serde_error() {
        // Arrange
        let msg = InvalidMessage {};
        let formatted = FormatMsg {
            message: msg,
            code: StatusCode::NOT_FOUND,
        };
        let request = gen_request("/api/404", None);

        // Act
        let result = formatted.respond_to(&request);

        // Assert
        assert!(result.error().is_some());
    }
}