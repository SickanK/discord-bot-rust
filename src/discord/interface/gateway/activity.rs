pub mod activity_assets;
pub mod activity_button;
pub mod activity_flags;
pub mod activity_party;
pub mod activity_secrets;
pub mod activity_timestamp;
pub mod activity_type;

use crate::discord::{interface::emoji::Emoji, snowflake::Snowflake};

use self::{
    activity_assets::ActivityAssets, activity_button::ActivityButton,
    activity_flags::ActivityFlags, activity_party::ActivityParty,
    activity_secrets::ActivitySecrets, activity_timestamp::ActivityTimestamp,
    activity_type::ActivityType,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Activity {
    name: String,
    #[serde(rename = "type")]
    activity_type: ActivityType,
    url: Option<String>,
    created_at: usize,
    timestamps: Option<ActivityTimestamp>,
    application_id: Option<Snowflake>,
    details: Option<String>,
    state: Option<String>,
    emoji: Option<Emoji>,
    party: Option<ActivityParty>,
    assets: Option<ActivityAssets>,
    secrets: Option<ActivitySecrets>,
    instance: Option<bool>,
    flags: Option<ActivityFlags>,
    buttons: Option<Vec<ActivityButton>>,
}
