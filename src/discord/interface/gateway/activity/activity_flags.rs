bitflags! {
pub struct ActivityFlags: u64 {
const INSTANCE = 1 << 0;
const JOIN = 1 << 1;
const SPECTATE = 1 << 2;
const JOIN_REQUEST = 1 << 3;
const SYNC = 1 << 4;
const PLAY = 1 << 5;
    }
}
struct ActivityFlagsVisitor;

impl<'de> serde::de::Visitor<'de> for ActivityFlagsVisitor {
    type Value = ActivityFlags;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("bits")
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match ActivityFlags::from_bits(v) {
            Some(f) => Ok(f),
            None => Err(E::custom("Failed to convert bits to flag")),
        }
    }
}

impl serde::ser::Serialize for ActivityFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u64(self.bits())
    }
}

impl<'de> serde::de::Deserialize<'de> for ActivityFlags {
    fn deserialize<D>(deserializer: D) -> Result<ActivityFlags, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_u16(ActivityFlagsVisitor)
    }
}
