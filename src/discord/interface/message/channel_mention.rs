use crate::discord::{interface::channel::channel_type::ChannelType, snowflake::Snowflake};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelMention {
    id: Snowflake,
    guild_id: Snowflake,
    #[serde(rename = "type")]
    channel_type: u8,
    name: String,
}
