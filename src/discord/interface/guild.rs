pub mod default_message_notifications;
pub mod explicit_content_filter;
pub mod guild_feature;
pub mod mfa_level;
pub mod nsfw_level;
pub mod premium_tier;
pub mod verification_level;
pub mod voice_state;
pub mod welcome_screen;
pub mod welcome_screen_channel;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::discord::snowflake::Snowflake;

use self::{
    default_message_notifications::DefaultMessageNotifications,
    explicit_content_filter::ExplicitContentFilter, guild_feature::GuildFeature,
    mfa_level::MFALevel, nsfw_level::NSFWLevel, premium_tier::PremiumTier,
    verification_level::VerificationLevel, voice_state::VoiceState, welcome_screen::WelcomeScreen,
};

use super::{
    channel::Channel,
    emoji::Emoji,
    gateway::presence_update::PresenceUpdate,
    message::{guild_member::GuildMember, role::Role, sticker::Sticker},
    stage_instance::StageInstance,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Guild {
    id: Snowflake,
    name: String,
    icon: Option<String>,
    icon_hash: Option<String>,
    splash: Option<String>,
    discovery_splash: Option<String>,
    owner: Option<bool>,
    owner_id: Snowflake,
    permissions: Option<String>,
    region: String,
    afk_channel_id: Option<Snowflake>,
    afk_timeout: usize,
    widget_enabled: Option<bool>,
    widget_channel_id: Option<Snowflake>,
    verification_level: u8,
    default_message_notifications: u8,
    explicit_content_filter: u8,
    roles: Option<Vec<Role>>,
    stickers: Option<Vec<Sticker>>,
    emojis: Option<Vec<Emoji>>,
    features: Option<Vec<GuildFeature>>,
    mfa_level: u8,
    application_id: Option<Snowflake>,
    system_channel_id: Option<Snowflake>,
    system_channel_flags: u8,
    rules_channel_id: Option<Snowflake>,
    joined_at: Option<DateTime<Utc>>,
    large: Option<bool>,
    unavailable: Option<bool>,
    member_count: Option<usize>,
    voice_states: Option<Vec<VoiceState>>,
    members: Option<Vec<GuildMember>>,
    channels: Option<Vec<Channel>>,
    threads: Option<Vec<Channel>>,
    presences: Option<Vec<PresenceUpdate>>,
    max_presences: Option<usize>,
    max_members: Option<usize>,
    vanity_url_code: Option<String>,
    description: Option<String>,
    banner: Option<String>,
    premium_tier: u8,
    premium_subscription_count: Option<usize>,
    preferred_locale: String,
    public_updates_channel_id: Option<Snowflake>,
    max_video_channel_users: Option<usize>,
    approximate_member_count: Option<usize>,
    approximate_presence_count: Option<usize>,
    welcome_screen: Option<WelcomeScreen>,
    nsfw_level: u8,
    stage_instances: Option<Vec<StageInstance>>,
}

bitflags! {
#[derive(Serialize, Deserialize)]
pub struct SystemChannelFlags: u32 {
    const RIP = 0;
const SUPPRESS_JOIN_NOTIFICATIONS = 1 << 0;
const SUPPRESS_PREMIUM_SUBSCRIPTIONS = 1 << 1;
const SUPPRESS_GUILD_REMINDER_NOTIFICATIONS = 1 << 2;
    }
}
