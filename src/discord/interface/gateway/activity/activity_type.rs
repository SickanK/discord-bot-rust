use num::ToPrimitive;
#[derive(ToPrimitive, FromPrimitive, Debug)]
pub enum ActivityType {
    Game = 0,
    Streaming = 1,
    Listening = 2,
    Watching = 3,
    Custom = 4,
    Competing = 5,
}
struct ActivityTypeVisitor;

impl<'de> serde::de::Visitor<'de> for ActivityTypeVisitor {
    type Value = ActivityType;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("u64")
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match num::FromPrimitive::from_u64(v) {
            Some(f) => Ok(f),
            None => Err(E::custom("Failed to convert to ActivityType")),
        }
    }
}

impl serde::ser::Serialize for ActivityType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u64(self.to_u64().unwrap())
    }
}

impl<'de> serde::de::Deserialize<'de> for ActivityType {
    fn deserialize<D>(deserializer: D) -> Result<ActivityType, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_u64(ActivityTypeVisitor)
    }
}
