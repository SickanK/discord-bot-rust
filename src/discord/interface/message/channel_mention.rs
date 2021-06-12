use crate::discord::{interface::channel::channel_type::ChannelType, snowflake::Snowflake};

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct ChannelMention {
    id: Snowflake,
    guild_id: Snowflake,
    #[serde(rename = "type")]
    channel_type: ChannelType,
    name: String,
}
