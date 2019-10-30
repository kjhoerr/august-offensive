use std::marker::Sized;

pub mod format_msg;
pub mod callback;
pub mod not_understood;

pub use self::format_msg::FormatMsg;
pub use self::callback::Callback;
pub use self::not_understood::NotUnderstood;

#[derive(Serialize)]
pub struct OutgoingMsg<T: Message> {
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
