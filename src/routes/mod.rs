use actix_web::{web::{route, scope, Query}, HttpRequest, Result, Scope};
use actix_web::http::StatusCode;
use messages::*;
use std::collections::HashMap;

mod callback;
mod not_understood;

use self::callback::callback;
use self::not_understood::not_understood;

type JsonMessage<U> = Result<FormatMsg<OutgoingMsg<U>>>;

// Provides the routes for the application
pub fn get_scope() -> Scope {
    scope("/api")
        .service(scope("/callback").default_service(route().to(callback)))
        .default_service(route().to(not_understood))
}

// Takes an HttpRequest path and splits it into an array.
fn destruct_path(path: &str) -> Vec<String> {
    path.split_terminator("/")
        // first element is always blank due to link starting with "/api"
        .skip(1)
        .map(String::from)
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::Method, test::TestRequest};
    use actix_web::{App, dev::Service, test::{block_on, init_service}};
    use actix_web::dev::{Body, ServiceResponse};
    use serde::Deserialize;
    use std::str;

    #[test]
    fn test_get_scope_callback() {
        // Arrange
        let req = TestRequest::with_uri("/api/callback").to_request();
        let scope = get_scope();
        let mut srv = init_service(App::new().service(scope));

        // Act
        let resp = &block_on(srv.call(req)).unwrap();

        // Assert
        assert_eq!(resp.status(), StatusCode::OK);

        let content = get_message::<OutgoingMsg<Callback>>(resp);
        assert_eq!(content.result_type, "CALLBACK");
        assert_eq!(content.content.path, vec!["api", "callback"]);
    }

    #[test]
    fn test_get_scope_not_understood() {
        // Arrange
        let req = TestRequest::with_uri("/api/404").to_request();
        let scope = get_scope();
        let mut srv = init_service(App::new().service(scope));

        // Act
        let resp = &block_on(srv.call(req)).unwrap();

        // Assert
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);

        let content = get_message::<OutgoingMsg<NotUnderstood>>(resp);
        assert_eq!(content.result_type, "NOT_UNDERSTOOD");
        assert_eq!(content.content.path, vec!["api", "404"]);
    }

    #[test]
    fn test_get_scope_blank() {
        // Arrange
        let req = TestRequest::with_uri("/").to_request();
        let scope = get_scope();
        let mut srv = init_service(App::new().service(scope));

        // Act
        let resp = block_on(srv.call(req)).unwrap();

        // Assert
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_destruct_path() {
        // Arrange
        let path = "/api/storm/breaking";

        // Act
        let result = destruct_path(path);

        // Assert
        assert_eq!(result, vec!["api", "storm", "breaking"]);
    }

    #[test]
    fn test_destruct_path_blank() {
        // Arrange
        let path = "/";

        // Act
        let result = destruct_path(path);

        // Assert
        assert!(result.is_empty());
    }

    #[test]
    fn test_destruct_path_empty() {
        // Arrange
        let path = "";

        // Act
        let result = destruct_path(path);

        // Assert
        assert!(result.is_empty());
    }

    pub fn gen_request(path: &str, method: Option<Method>) -> HttpRequest {
        TestRequest::with_uri(path)
            .method(method.unwrap_or(Method::GET))
            .to_http_request()
    }

    pub fn gen_query(map: &HashMap<String, String>) -> Query<HashMap<String, String>> {
        let mut query_str = String::new();
        for (key, val) in map.iter() {
            query_str.push_str(&format!("&{}={}", key, val));
        }

        Query::from_query(&query_str).unwrap()
    }

    fn get_message<'a, T: Deserialize<'a>>(response: &'a ServiceResponse) -> T {
        let body = response.response().body().as_ref().unwrap();
        let mut array = &[b'0';0][..];
        match body {
            Body::Bytes(b) => {
                array = b.as_ref();
            },
            _ => {},
        };

        let van = str::from_utf8(array).unwrap();
        serde_json::from_str(van).unwrap()
    }
}
