pub mod privacy_level;

use crate::discord::snowflake::Snowflake;

#[derive(Serialize, Deserialize, Debug)]
pub struct StageInstance {
    id: Snowflake,
    guild_id: Snowflake,
    channel_id: Snowflake,
    topic: String,
    privacy_level: u8,
    discoverable_disabled: bool,
}
