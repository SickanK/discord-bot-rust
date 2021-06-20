use crate::{discord::interface::message::Message, websockets::frame::WSFrame};
use serde_json;
use std::str::FromStr;

#[derive(Serialize, Deserialize)]
struct Type {
    t: String,
}

#[derive(EnumString, PartialEq, Debug)]
pub enum GatewayEvent {
    #[strum(serialize = "HELLO")]
    Hello,
    #[strum(serialize = "READY")]
    Ready,
    #[strum(serialize = "RESUMED")]
    Resumed,
    #[strum(serialize = "RECONNECT")]
    Reconnect,
    #[strum(serialize = "INVALID_SESSION")]
    InvalidSession,
    #[strum(serialize = "APPLICATION_COMMAND_CREATE")]
    ApplicationCommandCreate,
    #[strum(serialize = "APPLICATION_COMMAND_UPDATE")]
    ApplicationCommandUpdate,
    #[strum(serialize = "APPLICATION_COMMAND_DELETE")]
    ApplicationCommandDelete,
    #[strum(serialize = "CHANNEL_CREATE")]
    ChannelCreate,
    #[strum(serialize = "CHANNEL_UPDATE")]
    ChannelUpdate,
    #[strum(serialize = "CHANNEL_DELETE")]
    ChannelDelete,
    #[strum(serialize = "CHANNEL_PINS_UPDATE")]
    ChannelPinsUpdate,
    #[strum(serialize = "THREAD_CREATE")]
    ThreadCreate,
    #[strum(serialize = "THREAD_UPDATE")]
    ThreadUpdate,
    #[strum(serialize = "THREAD_DELETE")]
    ThreadDelete,
    #[strum(serialize = "THREAD_LIST_SYNC")]
    ThreadListSync,
    #[strum(serialize = "GUILD_CREATE")]
    GuildCreate,
    #[strum(serialize = "GUILD_UPDATE")]
    GuildUpdate,
    #[strum(serialize = "GUILD_DELETE")]
    GuildDelete,
    #[strum(serialize = "GUILD_BAN_ADD")]
    GuildBanAdd,
    #[strum(serialize = "GUILD_BAN_REMOVE")]
    GuildBanRemove,
    #[strum(serialize = "GUILD_EMOJIS_UPDATE")]
    GuildEmojisUpdate,
    #[strum(serialize = "GUILD_INTEGRATIONS_UPDATE")]
    GuildIntegrationsUpdate,
    #[strum(serialize = "GUILD_MEMBER_ADD")]
    GuildMemberAdd,
    #[strum(serialize = "GUILD_MEMBER_REMOVE")]
    GuildMemberRemove,
    #[strum(serialize = "GUILD_MEMBER_UPDATE")]
    GuildMemberUpdate,
    #[strum(serialize = "GUILD_MEMBERS_CHUNK")]
    GuildMembersChunk,
    #[strum(serialize = "GUILD_ROLE_CREATED")]
    GuildRoleCreated,
    #[strum(serialize = "GUILD_ROLE_UPDATE")]
    GuildRoleUpdate,
    #[strum(serialize = "GUILD_ROLE_DELETE")]
    GuildRoleDelete,
    #[strum(serialize = "INTEGRATION_CREATE")]
    IntegrationCreate,
    #[strum(serialize = "INTEGRATION_UPDATE")]
    IntegrationUpdate,
    #[strum(serialize = "INTEGRATION_DELETE")]
    IntegrationDelete,
    #[strum(serialize = "INTERACTION_CREATE")]
    InteractionCreate,
    #[strum(serialize = "INVITE_CREATE")]
    InviteCreate,
    #[strum(serialize = "INVITE_DELETE")]
    InviteDelete,
    #[strum(serialize = "MESSAGE_CREATE")]
    MessageCreate,
    #[strum(serialize = "MESSAGE_UPDATE")]
    MessageUpdate,
    #[strum(serialize = "MESSAGE_DELETE")]
    MessageDelete,
    #[strum(serialize = "MESSAGE_DELETE_BULK")]
    MessageDeleteBulk,
    #[strum(serialize = "MESSAGE_REACTION_ADD")]
    MessageReactionAdd,
    #[strum(serialize = "MESSAGE_REACTION_REMOVE")]
    MessageReactionRemove,
    #[strum(serialize = "MESSAGE_REACTION_REMOVE_ALL")]
    MessageReactionRemoveAll,
    #[strum(serialize = "MESSAGE_REACTIONREMOVE_EMOJI")]
    MessageReactionRemoveEmoji,
    #[strum(serialize = "PRESENCE_UPDATE")]
    PresenceUpdate,
    #[strum(serialize = "TYPING_START")]
    TypingStart,
    #[strum(serialize = "USER_UPDATE")]
    UserUpdate,
    #[strum(serialize = "VOICE_STATE_UPDATE")]
    VoiceStateUpdate,
    #[strum(serialize = "VOICE_SERVER_UPDATE")]
    VoiceServerUpdate,
    #[strum(serialize = "WEBHOCKS_UPDATE")]
    WebhocksUpdate,
}

impl GatewayEvent {
    pub fn from_frame(frame: WSFrame) -> Result<Self, String> {
        println!("{}", &frame.payload);
        let dispatch_type: Type = serde_json::from_str(&frame.payload).unwrap();
        Ok(GatewayEvent::from_str(&dispatch_type.t).unwrap())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscordEvent<A> {
    t: String,
    s: Option<u32>,
    op: Option<u8>,
    d: A,
}

pub struct Stuff {
    pub gte: GatewayEvent,
    pub frame: WSFrame,
}

impl Stuff {
    pub fn run(&self) {
        if self.gte == GatewayEvent::MessageCreate {
            //println!("{}", &self.frame.payload);
            let test: DiscordEvent<Message> = serde_json::from_str(&self.frame.payload).unwrap();
        //println!("{:?}", test);
        } else {
            println!("{:?}", &self.gte);
        }
    }
}
