pub mod application_flags;
pub mod membership_state;
pub mod team;

use crate::discord::{interface::user::User, snowflake::Snowflake};

use self::{application_flags::ApplicationFlags, team::Team};

#[derive(Serialize, Deserialize, Debug)]
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
