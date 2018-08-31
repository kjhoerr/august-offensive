use std::{collections::HashMap, marker::Sized};

#[derive(Serialize)]
pub struct OutgoingMsg<T> {
    pub result_type: String,
    pub content: T,
}

pub trait Message {
    fn name(&self) -> String;
    fn as_outgoing(self) -> OutgoingMsg<Self>
    where
        Self: Sized,
    {
        OutgoingMsg {
            result_type: self.name(),
            content: self,
        }
    }
}

#[derive(Serialize)]
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

#[derive(Serialize)]
pub struct NotUnderstood {
    pub path: Vec<String>,
}

impl Message for NotUnderstood {
    fn name(&self) -> String {
        String::from("NOT_UNDERSTOOD")
    }
}
