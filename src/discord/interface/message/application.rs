pub mod team;

use serde::{Deserialize, Serialize};

use crate::discord::{interface::user::User, snowflake::Snowflake};

use self::team::Team;

#[derive(Serialize, Deserialize)]
pub struct Application {
    id: Snowflake,
    name: String,
    icon: Option<String>,
    description: String,
    rpc_origins: Option<Vec<String>>,
    bot_public: bool,
    bot_require_code_grant: bool,
    terms_of_service_url: Option<String>,
    privacy_policy_url: Option<String>,
    owner: User,
    summary: String,
    verify_key: String,
    team: Team,
    guild_id: Option<Snowflake>,
    primary_sku_id: Option<Snowflake>,
    slug: Option<String>,
    cover_image: Option<String>,
    flags: ApplicationFlags,
}

bitflags! {
#[derive(Serialize, Deserialize)]
pub struct ApplicationFlags: u32 {
    const GATEWAY_PRESENCE = 1 << 12;
    const GATEWAY_PRESENCE_LIMITED = 1 << 13;
    const GATEWAY_GUILD_MEMBERS = 1 << 14;
    const GATEWAY_GUILD_MEMBERS_LIMITED = 1 << 15;
    const VERIFICATION_PENDING_GUILD_LIMIT = 1 << 16;
    const EMBEDDED = 1 << 17;
    }
}
