pub mod activity_assets;
pub mod activity_button;
pub mod activity_party;
pub mod activity_secrets;
pub mod activity_timestamp;
pub mod activity_type;

use serde::{Deserialize, Serialize};

use crate::discord::{interface::emoji::Emoji, snowflake::Snowflake};

use self::{
    activity_assets::ActivityAssets, activity_button::ActivityButton,
    activity_party::ActivityParty, activity_secrets::ActivitySecrets,
    activity_timestamp::ActivityTimestamp, activity_type::ActivityType,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Activity {
    name: String,
    #[serde(rename = "type")]
    activity_type: ActivityType, // activity type
    url: Option<String>,
    created_at: usize,
    timestamps: Option<ActivityTimestamp>, // timestamps
    application_id: Option<Snowflake>,
    details: Option<String>,
    state: Option<String>,
    emoji: Option<Emoji>,
    party: Option<ActivityParty>,     // party
    assets: Option<ActivityAssets>,   // assets
    secrets: Option<ActivitySecrets>, // secrets
    instance: Option<bool>,
    flags: Option<ActivityFlags>,         // activity flags
    buttons: Option<Vec<ActivityButton>>, // activity buttons
}

bitflags! {
#[derive(Serialize, Deserialize)]
pub struct ActivityFlags: u32 {
const INSTANCE = 1 << 0;
const JOIN = 1 << 1;
const SPECTATE = 1 << 2;
const JOIN_REQUEST = 1 << 3;
const SYNC = 1 << 4;
const PLAY = 1 << 5;
    }
}
