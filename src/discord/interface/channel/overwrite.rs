use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};

use crate::discord::snowflake::Snowflake;

#[derive(Serialize, Deserialize, Debug)]
pub struct Overwrite {
    id: Snowflake,
    #[serde(rename = "type")]
    overwrite_type: u8,
    allow: String,
    deny: String,
}

#[derive(Serialize, Deserialize, FromPrimitive, Debug)]
pub enum OverwriteType {
    Role = 0,
    Member = 1,
}
