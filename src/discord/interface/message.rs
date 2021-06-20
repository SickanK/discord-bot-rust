pub mod attachment;
pub mod channel_mention;
pub mod embed;
pub mod guild_member;
pub mod interaction;
pub mod message_activity;
pub mod message_component;
pub mod message_flags;
pub mod message_reference;
pub mod message_type;
pub mod reaction;
pub mod role;
pub mod sticker;

use crate::discord::snowflake::Snowflake;
use chrono::{DateTime, Utc};

use self::{
    attachment::Attachement, channel_mention::ChannelMention, embed::Embed,
    guild_member::GuildMember, interaction::Interaction, message_activity::MessageActivity,
    message_component::MessageComponent, message_flags::MessageFlags,
    message_reference::MessageReference, message_type::MessageType, reaction::Reaction, role::Role,
    sticker::Sticker,
};

use super::{application::Application, user::User};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    id: Snowflake,
    channel_id: Snowflake,
    guild_id: Option<Snowflake>,
    author: User,
    member: GuildMember,
    content: String,
    timestamp: Option<DateTime<Utc>>,
    edited_timestamp: Option<DateTime<Utc>>,
    tts: bool,
    mention_everyone: bool,
    mentions: Option<Vec<User>>,
    mention_roles: Option<Vec<Role>>,
    mention_channels: Option<Vec<ChannelMention>>,
    attachments: Vec<Attachement>,
    embeds: Option<Vec<Embed>>,
    reactions: Option<Vec<Reaction>>,
    nonce: Option<String>,
    pinned: bool,
    webhook_id: Option<Snowflake>,
    #[serde(rename = "type")]
    message_type: MessageType,
    activity: Option<MessageActivity>,
    application: Option<Application>,
    application_id: Option<Snowflake>,
    message_reference: Option<MessageReference>,
    flags: Option<u16>,
    stickers: Option<Vec<Sticker>>,
    referenced_message: Option<Box<Message>>,
    interaction: Option<Interaction>,
    thread: Option<Box<Message>>,
    components: Option<Vec<MessageComponent>>,
}
