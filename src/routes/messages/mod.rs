use std::marker::Sized;

pub mod callback;
pub mod not_understood;

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
