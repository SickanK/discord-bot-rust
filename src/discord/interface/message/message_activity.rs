use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MessageActivity {
    #[serde(rename = "type")]
    activity_type: ChannelType,
    party_id: Option<String>,
}

#[derive(FromPrimitive, Serialize, Deserialize)]
pub enum ChannelType {
    JOIN = 1,
    SPECTATE = 2,
    LISTEN = 3,
    JOIN_REQUEST = 5,
}
