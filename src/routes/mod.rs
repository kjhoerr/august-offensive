use actix_web::{web::Json, web::Query, HttpRequest, Result};
use std::collections::HashMap;

pub mod messages;
use messages::{callback::*, not_understood::*, *};

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

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::Method, test::TestRequest};

    #[test]
    fn test_not_understood() {
        // Arrange
        let uri = "/api/phpmyadmin/index.rs";
        let req = gen_request(uri, None);

        // Act
        let result = not_understood(req);

        // Assert
        assert_eq!(result.is_ok(), true);

        let val = result.unwrap().into_inner();
        assert_eq!(val.result_type, "NOT_UNDERSTOOD");
        assert_eq!(val.content.path, vec!["api", "phpmyadmin", "index.rs"]);
    }

    #[test]
    fn test_not_understood_blank() {
        // Arrange
        let uri = "/";
        let req = gen_request(uri, None);

        // Act
        let result = not_understood(req);

        // Assert
        assert_eq!(result.is_ok(), true);

        let val = result.unwrap().into_inner();
        assert_eq!(val.result_type, "NOT_UNDERSTOOD");
        assert_eq!(val.content.path.is_empty(), true);
    }

    #[test]
    fn test_callback_get() {
        // Arrange
        let uri = "/api/phpmyadmin/index.rs";
        let req = gen_request(uri, None);

        let mut ref_map = HashMap::new();
        ref_map.insert("hello".to_string(), "world".to_string());
        ref_map.insert("id".to_string(), "10011".to_string());
        let query = gen_query(&ref_map);

        // Act
        let result = callback(req, query);

        // Assert
        assert_eq!(result.is_ok(), true);

        let val = result.unwrap().into_inner();
        assert_eq!(val.result_type, "CALLBACK");
        assert_eq!(val.content.path, vec!["api", "phpmyadmin", "index.rs"]);
        assert_eq!(val.content.request, "GET");
        assert_eq!(val.content.content, ref_map);
    }

    #[test]
    fn test_callback_post() {
        // Arrange
        let uri = "/api/phpmyadmin/index.rs";
        let req = gen_request(uri, Some(Method::POST));

        let mut ref_map = HashMap::new();
        ref_map.insert("hello".to_string(), "world".to_string());
        ref_map.insert("id".to_string(), "10012".to_string());
        let query = gen_query(&ref_map);

        // Act
        let result = callback(req, query);

        // Assert
        assert_eq!(result.is_ok(), true);

        let val = result.unwrap().into_inner();
        assert_eq!(val.result_type, "CALLBACK");
        assert_eq!(val.content.path, vec!["api", "phpmyadmin", "index.rs"]);
        assert_eq!(val.content.request, "POST");
        assert_eq!(val.content.content, ref_map);
    }

    #[test]
    fn test_callback_blank() {
        // Arrange
        let uri = "/";
        let req = gen_request(uri, None);
        let query = Query::from_query("").unwrap();

        // Act
        let result = callback(req, query);

        // Assert
        assert_eq!(result.is_ok(), true);

        let val = result.unwrap().into_inner();
        assert_eq!(val.result_type, "CALLBACK");
        assert_eq!(val.content.path.is_empty(), true);
        assert_eq!(val.content.request, "GET");
        assert_eq!(val.content.content.is_empty(), true);
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
        assert_eq!(result.is_empty(), true);
    }

    #[test]
    fn test_destruct_path_empty() {
        // Arrange
        let path = "";

        // Act
        let result = destruct_path(path);

        // Assert
        assert_eq!(result.is_empty(), true);
    }

    fn gen_request(path: &str, method: Option<Method>) -> HttpRequest {
        TestRequest::with_uri(path)
            .method(method.unwrap_or(Method::GET))
            .to_http_request()
    }

    fn gen_query(map: &HashMap<String, String>) -> Query<HashMap<String, String>> {
        let mut query_str = String::new();
        for (key, val) in map.iter() {
            query_str.push_str(&format!("&{}={}", key, val));
        }

        Query::from_query(&query_str).unwrap()
    }
}
