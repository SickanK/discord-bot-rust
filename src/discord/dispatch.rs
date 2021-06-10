use crate::websockets::frame::WSFrame;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct Type {
    t: String,
}
pub enum DiscordDispatch {
    MessageCreate,
    Ready,
}

impl<'a> DiscordDispatch {
    pub fn run(&self) {
        print!("Just ran!")
    }

    pub fn from_frame(frame: WSFrame) -> Result<Self, String> {
        let dispatch_type: Type = serde_json::from_str(&frame.payload).unwrap();
        Self::from_str(&dispatch_type.t)
    }

    pub fn from_str(dispatch_type: &str) -> Result<Self, String> {
        match dispatch_type {
            "MESSAGE_CREATE" => Ok(DiscordDispatch::MessageCreate),
            "READY" => Ok(DiscordDispatch::Ready),
            _ => Err(format!("{}: Dispatch type does not exist", dispatch_type)),
        }
    }
}
