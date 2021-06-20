pub mod premium_type;
pub mod user_flags;

use crate::discord::snowflake::Snowflake;

use self::user_flags::UserFlags;

#[derive(Serialize, Deserialize, Debug)]
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
    premium_type: Option<usize>,
    public_flags: Option<usize>,
}
