bitflags! {
pub struct MessageFlags: u32 {
const CROSSPOSTED = 1 << 0;
const IS_CROSSPOST = 1 << 1;
const SUPPRESS_EMBEDS = 1 << 2;
const SOURCE_MESSAGE_DELETED = 1 << 3;
const URGENT = 1 << 4;
const HAS_THREAD = 1 << 5;
const EPHEMERAL = 1 << 6;
const LOADING = 1 << 7;
    }
}
struct MessageFlagsVisitor;

impl<'de> serde::de::Visitor<'de> for MessageFlagsVisitor {
    type Value = MessageFlags;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("bits")
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match MessageFlags::from_bits(v) {
            Some(f) => Ok(f),
            None => Err(E::custom("Failed to convert bits to flag")),
        }
    }
}

impl serde::ser::Serialize for MessageFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.bits())
    }
}

impl<'de> serde::de::Deserialize<'de> for MessageFlags {
    fn deserialize<D>(deserializer: D) -> Result<MessageFlags, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_u16(MessageFlagsVisitor)
    }
}
