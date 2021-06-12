pub mod channel_type;
pub mod overwrite;
pub mod thread_member;
pub mod thread_metadata;
pub mod video_quality_mode;

use crate::discord::snowflake::Snowflake;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use self::{
    channel_type::ChannelType, overwrite::Overwrite, thread_member::ThreadMember,
    thread_metadata::ThreadMetadata, video_quality_mode::VideoQualityMode,
};

use super::user::User;

#[derive(Serialize, Deserialize, Debug)]
pub struct Channel {
    id: Snowflake,
    #[serde(rename = "type")]
    channel_type: u8,
    guild_id: Option<Snowflake>,
    position: Option<u32>,
    permission_overwrites: Option<Vec<Overwrite>>,
    name: Option<String>,
    topic: Option<String>,
    nsfw: Option<bool>,
    last_message_id: Option<Snowflake>,
    bitrate: Option<usize>,
    user_limit: Option<u32>,
    rate_limit_per_user: Option<u32>,
    recipients: Option<Vec<User>>,
    icon: Option<String>,
    owner_id: Option<Snowflake>,
    application_id: Option<Snowflake>,
    parent_id: Option<Snowflake>,
    last_pin_timestamp: Option<DateTime<Utc>>,
    rtc_region: Option<String>,
    video_quality_mode: Option<u8>,
    message_count: Option<u16>,
    member_count: Option<u16>,
    thread_metadata: Option<ThreadMetadata>,
    member: Option<ThreadMember>,
}
