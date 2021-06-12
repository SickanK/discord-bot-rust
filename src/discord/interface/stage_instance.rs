pub mod privacy_level;

use serde::{Deserialize, Serialize};

use crate::discord::snowflake::Snowflake;

use self::privacy_level::PrivacyLevel;

#[derive(Serialize, Deserialize, Debug)]
pub struct StageInstance {
    id: Snowflake,
    guild_id: Snowflake,
    channel_id: Snowflake,
    topic: String,
    privacy_level: PrivacyLevel,
    discoverable_disabled: bool,
}
