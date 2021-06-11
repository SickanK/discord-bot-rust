use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};

use crate::discord::{interface::user::User, snowflake::Snowflake};
#[derive(Serialize, Deserialize)]
pub struct Interaction {
    id: Snowflake,
    #[serde(rename = "type")]
    interaction_type: InteractionType,
    name: String,
    user: User,
}

#[derive(FromPrimitive, Serialize, Deserialize)]
pub enum InteractionType {
    Ping = 1,
    ApplicationCommand = 2,
    MessageComponent = 3,
}
