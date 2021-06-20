use crate::discord::snowflake::Snowflake;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug)]
pub struct ThreadMetadata {
    archived: bool,
    archiver_id: Option<Snowflake>,
    auto_archive_duration: u32,
    archive_timestamp: DateTime<Utc>,
    locked: Option<bool>,
}
