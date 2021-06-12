use serde::{Deserialize, Serialize};

use crate::discord::snowflake::Snowflake;

#[derive(Serialize, Deserialize, Debug)]
pub struct WelcomeScreenChannel {
    channel_id: Snowflake,
    description: String,
    emoji_id: Option<Snowflake>,
    emoji_name: Option<String>,
}
