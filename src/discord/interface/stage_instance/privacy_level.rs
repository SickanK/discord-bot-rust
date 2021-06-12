use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, FromPrimitive, Debug)]
pub enum PrivacyLevel {
    Public = 1,
    GuildOnly = 2,
}
