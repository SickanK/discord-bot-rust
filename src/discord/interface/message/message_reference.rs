use crate::discord::snowflake::Snowflake;
#[derive(Serialize, Deserialize, Debug)]
pub struct MessageReference {
    message_id: Option<Snowflake>,
    channel_id: Option<Snowflake>,
    guild_id: Option<Snowflake>,
    fail_if_not_exists: Option<bool>,
}
