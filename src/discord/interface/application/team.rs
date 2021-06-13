pub mod team_member;

use crate::discord::snowflake::Snowflake;

use self::team_member::TeamMember;

#[derive(Serialize, Deserialize, Debug)]
pub struct Team {
    icon: Option<String>,
    id: Snowflake,
    members: TeamMember,
    name: String,
    owner_user_id: Snowflake,
}
