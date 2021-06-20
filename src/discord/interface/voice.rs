use chrono::{DateTime, Utc};

use crate::discord::snowflake::Snowflake;

use super::message::guild_member::GuildMember;

#[derive(Serialize, Deserialize)]
pub struct Voice {
    guild_id: Option<Snowflake>,
    channel_id: Option<Snowflake>,
    user_id: Snowflake,
    member: Option<GuildMember>,
    session_id: String,
    deaf: bool,
    mute: bool,
    self_deaf: bool,
    self_mute: bool,
    self_stream: Option<bool>,
    self_video: bool,
    suppress: bool,
    request_to_speak_timestamp: Option<DateTime<Utc>>,
}
