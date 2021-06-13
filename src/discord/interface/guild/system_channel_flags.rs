bitflags! {
pub struct SystemChannelFlags: u32 {
    const SUPPRESS_JOIN_NOTIFICATIONS = 1 << 0;
    const SUPPRESS_PREMIUM_SUBSCRIPTIONS = 1 << 1;
    const SUPPRESS_GUILD_REMINDER_NOTIFICATIONS = 1 << 2;
    }
}
struct SystemChannelFlagsVisitor;

impl<'de> serde::de::Visitor<'de> for SystemChannelFlagsVisitor {
    type Value = SystemChannelFlags;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("bits")
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match SystemChannelFlags::from_bits(v) {
            Some(f) => Ok(f),
            None => Err(E::custom("Failed to convert bits to flag")),
        }
    }
}

impl serde::ser::Serialize for SystemChannelFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.bits())
    }
}

impl<'de> serde::de::Deserialize<'de> for SystemChannelFlags {
    fn deserialize<D>(deserializer: D) -> Result<SystemChannelFlags, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_u16(SystemChannelFlagsVisitor)
    }
}
