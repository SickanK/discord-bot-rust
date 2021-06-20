use num::ToPrimitive;
#[derive(ToPrimitive, FromPrimitive)]
pub enum MessageComponentType {
    ActionRow = 1,
    Button = 2,
}
struct MessageComponentTypeVisitor;

impl<'de> serde::de::Visitor<'de> for MessageComponentTypeVisitor {
    type Value = MessageComponentType;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("u64")
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match num::FromPrimitive::from_u64(v) {
            Some(f) => Ok(f),
            None => Err(E::custom("Failed to convert to MessageComponentType")),
        }
    }
}

impl serde::ser::Serialize for MessageComponentType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u64(self.to_u64().unwrap())
    }
}

impl<'de> serde::de::Deserialize<'de> for MessageComponentType {
    fn deserialize<D>(deserializer: D) -> Result<MessageComponentType, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_u64(MessageComponentTypeVisitor)
    }
}
