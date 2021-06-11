use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};

use crate::discord::snowflake::Snowflake;
#[derive(Serialize, Deserialize)]
pub struct Sticker {
    id: Snowflake,
    pack_id: Snowflake,
    name: String,
    description: String,
    tags: Option<String>,
    asset: String,
    format_type: FormatType,
}

#[derive(FromPrimitive, Serialize, Deserialize)]
pub enum FormatType {
    PNG = 1,
    APNG = 2,
    LOTTIE = 3,
}
