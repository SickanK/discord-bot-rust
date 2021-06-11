use crate::discord::snowflake::Snowflake;

use super::channel_type::ChannelType;

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct ChannelMention {
    id: Snowflake,
    guild_id: Snowflake,
    #[serde(rename = "type")]
    channel_type: ChannelType,
    name: String,
}
