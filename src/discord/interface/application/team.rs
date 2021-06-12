use crate::discord::{interface::user::User, snowflake::Snowflake};
use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Team {
    icon: Option<String>,
    id: Snowflake,
    members: TeamMember,
    name: String,
    owner_user_id: Snowflake,
}

#[derive(Serialize, Deserialize)]
pub struct TeamMember {
    membership_state: u8,
    permissions: Option<Vec<String>>,
    team_id: Snowflake,
    user: User,
}

#[derive(FromPrimitive, Serialize, Deserialize)]
pub enum MembershipState {
    Invited = 1,
    Accepted = 2,
}
