use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};

use crate::discord::snowflake::Snowflake;
#[derive(Serialize, Deserialize, Debug)]
pub struct Sticker {
    id: Snowflake,
    pack_id: Snowflake,
    name: String,
    description: String,
    tags: Option<String>,
    asset: String,
    format_type: u8,
}

#[derive(FromPrimitive, Serialize, Deserialize)]
pub enum FormatType {
    Png = 1,
    APng = 2,
    Lottie = 3,
}
