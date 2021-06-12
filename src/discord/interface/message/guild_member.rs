use chrono::{DateTime, Utc};

use crate::discord::{interface::user::User, snowflake::Snowflake};

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct GuildMember {
    user: Option<User>,
    nick: Option<String>,
    roles: Option<Vec<Snowflake>>,
    joined_at: DateTime<Utc>,             // iso8601
    premium_since: Option<DateTime<Utc>>, // iso8601
    deaf: bool,
    mute: bool,
    pending: Option<bool>,
    permissions: Option<String>,
}
