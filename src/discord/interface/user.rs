use crate::discord::snowflake::Snowflake;
use num_derive::FromPrimitive;

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct User {
    id: Snowflake,
    username: String,
    discriminator: String,
    avatar: Option<String>,
    bot: Option<bool>,
    system: Option<bool>,
    mfa_enabled: Option<bool>,
    locale: Option<String>,
    verified: Option<bool>,
    email: Option<String>,
    flags: Option<UserFlags>,
    premium_type: Option<PremiumType>,
    public_flags: Option<UserFlags>,
}

bitflags! {
#[derive(Serialize, Deserialize)]
pub struct UserFlags: u32 {
        const NONE = 0;
        const DISCORD_EMPLOYE = 1 << 0;
        const PARTNERED_SERVER_OWNER = 1 << 1;
        const HYPESQUAD_EVENTS = 1 << 2;
        const BUG_HUNTER_LEVEL_1 = 1 << 3;
        const HOUSE_BRAVERY = 1 << 6;
        const HOUSE_BRILLIANCE = 1 << 7;
        const HOUSE_BALANCE = 1 << 8;
        const EARLY_SUPPORTER = 1 << 9;
        const TEAM_USER = 1 << 10;
        const BUG_HUNTER_LEVEL_2 = 1 << 14;
        const VERIFIED_BOT = 1 << 16;
        const EARLY_VERIFIED_BOT_DEVELOPER = 1 << 17;
        const DISCORD_CERTIFIED_MODERATOR = 1 << 18;
    }
}

#[derive(FromPrimitive, Serialize, Deserialize)]
pub enum PremiumType {
    None = 0,
    NitroClassic,
    Nitro,
}
