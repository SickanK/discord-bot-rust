bitflags! {
pub struct ApplicationFlags: u32 {
    const GATEWAY_PRESENCE = 1 << 12;
    const GATEWAY_PRESENCE_LIMITED = 1 << 13;
    const GATEWAY_GUILD_MEMBERS = 1 << 14;
    const GATEWAY_GUILD_MEMBERS_LIMITED = 1 << 15;
    const VERIFICATION_PENDING_GUILD_LIMIT = 1 << 16;
    const EMBEDDED = 1 << 17;
    }
}
struct ApplicationFlagsVisitor;

impl<'de> serde::de::Visitor<'de> for ApplicationFlagsVisitor {
    type Value = ApplicationFlags;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("bits")
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match ApplicationFlags::from_bits(v) {
            Some(f) => Ok(f),
            None => Err(E::custom("Failed to convert bits to flag")),
        }
    }
}

impl serde::ser::Serialize for ApplicationFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.bits())
    }
}

impl<'de> serde::de::Deserialize<'de> for ApplicationFlags {
    fn deserialize<D>(deserializer: D) -> Result<ApplicationFlags, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_u16(ApplicationFlagsVisitor)
    }
}
