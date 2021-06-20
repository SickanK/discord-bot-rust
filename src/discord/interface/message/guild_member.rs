use chrono::{DateTime, Utc};

use crate::discord::{interface::user::User, snowflake::Snowflake};

#[derive(Serialize, Deserialize, Debug)]
pub struct GuildMember {
    user: Option<User>,
    nick: Option<String>,
    roles: Option<Vec<Snowflake>>,
    joined_at: DateTime<Utc>,
    premium_since: Option<DateTime<Utc>>,
    deaf: bool,
    mute: bool,
    pending: Option<bool>,
    permissions: Option<String>,
}
