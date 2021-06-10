use crate::websockets::frame::WSFrame;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct Type {
    t: String,
}
pub enum GatewayEvents {
    Hello,
    Ready,
    Resumed,
    Reconnect,
    InvalidSession,
    ApplicationCommandCreate,
    ApplicationCommandUpdate,
    ApplicationCommandDelete,
    ChannelCreate,
    ChannelUpdate,
    ChannelDelete,
    ChannelPinsUpdate,
    ThreadCreate,
    ThreadUpdate,
    ThreadDelete,
    ThreadListSync,
    GuildCreate,
    GuildUpdate,
    GuildDelete,
    GuildBanAdd,
    GuildBanRemove,
    GuildEmojisUpdate,
    GuildIntegrationsUpdate,
    GuildMemberAdd,
    GuildMemberRemove,
    GuildMemberUpdate,
    GuildMembersChunk,
    GuildRoleCreated,
    GuildRoleUpdate,
    GuildRoleDelete,
    IntegrationCreate,
    IntegrationUpdate,
    IntegrationDelete,
    InteractionCreate,
    InviteCreate,
    InviteDelete,
    MessageCreate,
    MessageDelete,
    MessageDeleteBulk,
    MessageReactionAdd,
    MessageReactionRemove,
    MessageReactionRemoveAll,
    MessageReactionRemoveEmoji,
    PresenceUpdate,
    TypingStart,
    UserUpdate,
    VoiceStateUpdate,
    VoiceServerUpdate,
    WebhocksUpdate,
}

impl<'a> GatewayEvents {
    pub fn run(&self) {
        println!("Just ran!")
    }

    pub fn from_frame(frame: WSFrame) -> Result<Self, String> {
        let dispatch_type: Type = serde_json::from_str(&frame.payload).unwrap();
        Self::from_str(&dispatch_type.t)
    }

    pub fn from_str(event_type: &str) -> Result<Self, String> {
        match event_type {
            "HELLO" => Ok(GatewayEvents::Hello),
            "READY" => Ok(GatewayEvents::Ready),
            "RESUMED" => Ok(GatewayEvents::Resumed),
            "RECONNECT" => Ok(GatewayEvents::Reconnect),
            "INVALID_SESSION" => Ok(GatewayEvents::InvalidSession),
            "APPLICATION_COMMAND_CREATE" => Ok(GatewayEvents::ApplicationCommandCreate),
            "APPLICATION_COMMAND_UPDATE" => Ok(GatewayEvents::ApplicationCommandUpdate),
            "APPLICATION_COMMAND_DELETE" => Ok(GatewayEvents::ApplicationCommandDelete),
            "CHANNEL_CREATE" => Ok(GatewayEvents::ChannelCreate),
            "CHANNEL_UPDATE" => Ok(GatewayEvents::ChannelUpdate),
            "CHANNEL_DELETE" => Ok(GatewayEvents::ChannelDelete),
            "CHANNEL_PINS_UPDATE" => Ok(GatewayEvents::ChannelPinsUpdate),
            "THREAD_CREATE" => Ok(GatewayEvents::ThreadCreate),
            "THREAD_UPDATE" => Ok(GatewayEvents::ThreadUpdate),
            "THREAD_DELETE" => Ok(GatewayEvents::ThreadDelete),
            "THREAD_LIST_SYNC" => Ok(GatewayEvents::ThreadListSync),
            "GUILD_CREATE" => Ok(GatewayEvents::GuildCreate),
            "GUILD_UPDATE" => Ok(GatewayEvents::GuildUpdate),
            "GUILD_DELETE" => Ok(GatewayEvents::GuildDelete),
            "GUILD_BAN_ADD" => Ok(GatewayEvents::GuildBanAdd),
            "GUILD_BAN_REMOVE" => Ok(GatewayEvents::GuildBanRemove),
            "GUILD_EMOJIS_UPDATE" => Ok(GatewayEvents::GuildEmojisUpdate),
            "GUILD_INTEGRATIONS_UPDATE" => Ok(GatewayEvents::GuildIntegrationsUpdate),
            "GUILD_MEMBER_ADD" => Ok(GatewayEvents::GuildMemberAdd),
            "GUILD_MEMBER_REMOVE" => Ok(GatewayEvents::GuildMemberRemove),
            "GUILD_MEMBER_UPDATE" => Ok(GatewayEvents::GuildMemberUpdate),
            "GUILD_MEMBERS_CHUNK" => Ok(GatewayEvents::GuildMembersChunk),
            "GUILD_ROLE_CREATED" => Ok(GatewayEvents::GuildRoleCreated),
            "GUILD_ROLE_UPDATE" => Ok(GatewayEvents::GuildRoleUpdate),
            "GUILD_ROLE_DELETE" => Ok(GatewayEvents::GuildRoleDelete),
            "INTEGRATION_CREATE" => Ok(GatewayEvents::IntegrationCreate),
            "INTEGRATION_UPDATE" => Ok(GatewayEvents::IntegrationUpdate),
            "INTEGRATION_DELETE" => Ok(GatewayEvents::IntegrationDelete),
            "INTERACTION_CREATE" => Ok(GatewayEvents::InteractionCreate),
            "INVITE_CREATE" => Ok(GatewayEvents::InviteCreate),
            "INVITE_DELETE" => Ok(GatewayEvents::InviteDelete),
            "MESSAGE_CREATE" => Ok(GatewayEvents::MessageCreate),
            "MESSAGE_DELETE" => Ok(GatewayEvents::MessageDelete),
            "MESSAGE_DELETE_BULK" => Ok(GatewayEvents::MessageDeleteBulk),
            "MESSAGE_REACTION_ADD" => Ok(GatewayEvents::MessageReactionAdd),
            "MESSAGE_REACTION_REMOVE" => Ok(GatewayEvents::MessageReactionRemove),
            "MESSAGE_REACTION_REMOVE_ALL" => Ok(GatewayEvents::MessageReactionRemoveAll),
            "MESSAGE_REACTIONREMOVE_EMOJI" => Ok(GatewayEvents::MessageReactionRemoveEmoji),
            "PRESENCE_UPDATE" => Ok(GatewayEvents::PresenceUpdate),
            "TYPING_START" => Ok(GatewayEvents::TypingStart),
            "USER_UPDATE" => Ok(GatewayEvents::UserUpdate),
            "VOICE_STATE_UPDATE" => Ok(GatewayEvents::VoiceStateUpdate),
            "VOICE_SERVER_UPDATE" => Ok(GatewayEvents::VoiceServerUpdate),
            "WEBHOCKS_UPDATE" => Ok(GatewayEvents::WebhocksUpdate),
            _ => Err(format!("{}: Dispatch type does not exist", event_type)),
        }
    }
}
