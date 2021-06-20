pub mod interaction_type;
use crate::discord::{interface::user::User, snowflake::Snowflake};

#[derive(Serialize, Deserialize, Debug)]
pub struct Interaction {
    id: Snowflake,
    #[serde(rename = "type")]
    interaction_type: u8,
    name: String,
    user: User,
}
