use messages::Message;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct Callback {
    pub path: Vec<String>,
    pub request: String,
    pub content: HashMap<String, String>,
}

impl Message for Callback {
    fn name(&self) -> String {
        String::from("CALLBACK")
    }
}

impl PartialEq for Callback {
    fn eq(&self, other: &Self) -> bool {
        self.request == other.request && self.path == other.path && self.content == other.content
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        // Arrange
        let query = HashMap::new();
        let message = Callback {
            path: vec![],
            request: String::from("GET"),
            content: query,
        };

        // Act
        let name = message.name();

        // Assert
        assert_eq!(name, "CALLBACK");
    }

    #[test]
    fn test_asoutgoing() {
        // Arrange
        let message = Callback {
            path: vec![],
            request: String::from("GET"),
            content: HashMap::new(),
        };

        // Act
        let outgoing = message.as_outgoing();

        // Assert
        assert_eq!(outgoing.result_type, "CALLBACK");
        assert_eq!(
            outgoing.content,
            Callback {
                path: vec![],
                request: String::from("GET"),
                content: HashMap::new(),
            }
        );
    }
}
