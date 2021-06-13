use crate::discord::{interface::user::User, snowflake::Snowflake};

use super::message::role::Role;

#[derive(Serialize, Deserialize, Debug)]
pub struct Emoji {
    id: Option<Snowflake>,
    name: Option<String>,
    roles: Option<Vec<Role>>,
    user: Option<User>,
    require_colons: Option<bool>,
    managed: Option<bool>,
    animated: Option<bool>,
    available: Option<bool>,
}
