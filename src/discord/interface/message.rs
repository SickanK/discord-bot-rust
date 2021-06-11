pub mod application;
pub mod attachment;
pub mod channel_mention;
pub mod channel_type;
pub mod embed;
pub mod emoji;
pub mod guild_member;
pub mod interaction;
pub mod message_activity;
pub mod message_component;
pub mod message_reference;
pub mod reaction;
pub mod role;
pub mod sticker;

use crate::discord::snowflake::{self, Snowflake};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use self::{
    application::Application, attachment::Attachement, channel_mention::ChannelMention,
    embed::Embed, guild_member::GuildMember, interaction::Interaction,
    message_activity::MessageActivity, message_component::MessageComponent,
    message_reference::MessageReference, reaction::Reaction, role::Role, sticker::Sticker,
};

use super::user::User;

#[derive(Serialize, Deserialize)]
pub struct Message {
    id: Snowflake,
    channel_id: Snowflake,
    guild_id: Option<Snowflake>,
    author: User,
    member: GuildMember,
    content: String,
    timestamp: DateTime<Utc>,
    edited_timestamp: Option<DateTime<Utc>>,
    tts: bool,
    mention_everyone: bool,
    mentions: Vec<User>,
    mention_roles: Vec<Role>,
    mention_channels: Option<Vec<ChannelMention>>,
    attachments: Vec<Attachement>,
    embeds: Vec<Embed>,
    reactions: Option<Vec<Reaction>>,
    nonce: Option<String>,
    pinned: bool,
    webhook_id: Option<Snowflake>,
    message_type: u8,
    activity: Option<MessageActivity>,
    application: Option<Application>,
    application_id: Option<Snowflake>,
    message_reference: Option<MessageReference>,
    flags: Option<MessageFlags>,
    stickers: Option<Vec<Sticker>>,
    referenced_message: Option<Box<Message>>,
    interaction: Option<Interaction>,
    thread: Option<Box<Message>>,
    components: Option<Vec<MessageComponent>>,
}

bitflags! {
#[derive(Serialize, Deserialize)]
pub struct MessageFlags: u32 {
const CROSSPOSTED = 1 << 0;
const IS_CROSSPOST = 1 << 1;
const SUPPRESS_EMBEDS = 1 << 2;
const SOURCE_MESSAGE_DELETED = 1 << 3;
const URGENT = 1 << 4;
const HAS_THREAD = 1 << 5;
const EPHEMERAL = 1 << 6;
const LOADING = 1 << 7;
    }
}
