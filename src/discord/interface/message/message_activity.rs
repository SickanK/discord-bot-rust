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
    Join = 1,
    Spectate = 2,
    Listen = 3,
    JoinRequest = 5,
}
