use crate::routes::*;

// Sends a default response message when requested an undefined resource.
pub async fn not_understood(req: HttpRequest) -> JsonMessage<NotUnderstood> {
    let message = NotUnderstood {
        path: destruct_path(req.path()),
    };

    Ok(FormatMsg {
        message: message.as_outgoing(),
        code: StatusCode::NOT_FOUND,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::routes::tests::*;

    #[actix_rt::test]
    async fn test_not_understood() {
        // Arrange
        let uri = "/api/phpmyadmin/index.rs";
        let req = gen_request(uri, None);

        // Act
        let result = not_understood(req).await;

        // Assert
        assert!(result.is_ok());

        let val = result.unwrap().into_inner();
        assert_eq!(val.result_type, "NOT_UNDERSTOOD");
        assert_eq!(val.content.path, vec!["api", "phpmyadmin", "index.rs"]);
    }

    #[actix_rt::test]
    async fn test_not_understood_blank() {
        // Arrange
        let uri = "/";
        let req = gen_request(uri, None);

        // Act
        let result = not_understood(req).await;

        // Assert
        assert!(result.is_ok());

        let val = result.unwrap().into_inner();
        assert_eq!(val.result_type, "NOT_UNDERSTOOD");
        assert!(val.content.path.is_empty());
    }
}