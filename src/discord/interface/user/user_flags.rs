bitflags! {
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
struct UserFlagsVisitor;

impl<'de> serde::de::Visitor<'de> for UserFlagsVisitor {
    type Value = UserFlags;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("bits")
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match UserFlags::from_bits(v) {
            Some(f) => Ok(f),
            None => Err(E::custom("Failed to convert bits to flag")),
        }
    }
}

impl serde::ser::Serialize for UserFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.bits())
    }
}

impl<'de> serde::de::Deserialize<'de> for UserFlags {
    fn deserialize<D>(deserializer: D) -> Result<UserFlags, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_u16(UserFlagsVisitor)
    }
}
