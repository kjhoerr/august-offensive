use crate::messages::Message;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NotUnderstood {
    pub path: Vec<String>,
}

impl Message for NotUnderstood {
    fn name(&self) -> String {
        String::from("NOT_UNDERSTOOD")
    }
}

impl PartialEq for NotUnderstood {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        // Arrange
        let message = NotUnderstood { path: vec![] };

        // Act
        let name = message.name();

        // Assert
        assert_eq!(name, "NOT_UNDERSTOOD");
    }

    #[test]
    fn test_asoutgoing() {
        // Arrange
        let message = NotUnderstood { path: vec![] };
        let message_ref = message.clone();

        // Act
        let outgoing = message.as_outgoing();

        // Assert
        assert_eq!(outgoing.result_type, "NOT_UNDERSTOOD");
        assert_eq!(outgoing.content, message_ref);
    }
}
