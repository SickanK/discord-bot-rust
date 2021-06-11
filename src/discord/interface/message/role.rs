use crate::discord::snowflake::Snowflake;

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Role {
    id: Snowflake,
    name: String,
    color: usize,
    hoist: bool,
    position: u16,
    permissions: String,
    managed: bool,
    mentionable: bool,
    tags: Option<RoleTags>,
}

#[derive(Serialize, Deserialize)]
pub struct RoleTags {
    bot_id: Option<Snowflake>,
    integration_id: Option<Snowflake>,
    premium_subscriber: Option<String>,
}
