use serde::{Deserialize, Serialize};

use crate::discord::{interface::user::User, snowflake::Snowflake};

use super::{activity::Activity, client_status::ClientStatus};

#[derive(Serialize, Deserialize, Debug)]
pub struct PresenceUpdate {
    user: User,
    guild_id: Snowflake,
    status: String,
    activities: Option<Vec<Activity>>,
    client_status: ClientStatus,
}
