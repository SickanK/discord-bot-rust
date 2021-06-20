use crate::discord::{
    interface::{application::membership_state::MembershipState, user::User},
    snowflake::Snowflake,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct TeamMember {
    membership_state: MembershipState,
    permissions: Option<Vec<String>>,
    team_id: Snowflake,
    user: User,
}
