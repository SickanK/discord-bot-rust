use chrono::{DateTime, Utc};

use crate::discord::snowflake::Snowflake;

#[derive(Serialize, Deserialize, Debug)]
pub struct ThreadMember {
    id: Snowflake,
    user_id: Snowflake,
    join_timestamp: DateTime<Utc>,
    flags: usize,
}
