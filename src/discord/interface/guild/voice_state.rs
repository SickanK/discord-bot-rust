use chrono::{DateTime, Utc};

use crate::discord::{interface::message::guild_member::GuildMember, snowflake::Snowflake};

#[derive(Serialize, Deserialize, Debug)]
pub struct VoiceState {
    guild_id: Option<Snowflake>,
    channel_id: Option<Snowflake>,
    user_id: Option<Snowflake>,
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
