use crate::routes::*;

// Sends Callback message with information from HttpRequest.
pub fn callback(req: HttpRequest, query: Query<HashMap<String, String>>) -> JsonMessage<Callback> {
    let path = req.path();
    let method = req.method().as_str();

    let callback = Callback {
        path: destruct_path(path),
        request: String::from(method),
        content: query.into_inner(),
    };

    Ok(FormatMsg::ok(callback.as_outgoing()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::routes::tests::*;
    use actix_web::http::Method;

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
        assert!(result.is_ok());

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
        assert!(result.is_ok());

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
        assert!(result.is_ok());

        let val = result.unwrap().into_inner();
        assert_eq!(val.result_type, "CALLBACK");
        assert!(val.content.path.is_empty());
        assert_eq!(val.content.request, "GET");
        assert!(val.content.content.is_empty());
    }
}