use std::collections::HashMap;

trait Content {}

trait Message {
    fn name(&self) -> String;
}

#[derive(Serialize)]
pub struct Callback {
    pub path: Vec<String>,
    pub request: HashMap<String, String>,
}

impl Message for Callback {
    fn name(&self) -> String {
        String::from("CALLBACK")
    }
}