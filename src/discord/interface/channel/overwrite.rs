use crate::discord::snowflake::Snowflake;
use num::ToPrimitive;

#[derive(Serialize, Deserialize, Debug)]
pub struct Overwrite {
    id: Snowflake,
    #[serde(rename = "type")]
    overwrite_type: u8,
    allow: String,
    deny: String,
}

#[derive(ToPrimitive, FromPrimitive, Debug)]
pub enum OverwriteType {
    Role = 0,
    Member = 1,
}

struct OverwriteTypeVisitor;

impl<'de> serde::de::Visitor<'de> for OverwriteTypeVisitor {
    type Value = OverwriteType;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("u64")
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match num::FromPrimitive::from_u64(v) {
            Some(f) => Ok(f),
            None => Err(E::custom("Failed to convert to OverwriteType")),
        }
    }
}

impl serde::ser::Serialize for OverwriteType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u64(self.to_u64().unwrap())
    }
}

impl<'de> serde::de::Deserialize<'de> for OverwriteType {
    fn deserialize<D>(deserializer: D) -> Result<OverwriteType, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_u64(OverwriteTypeVisitor)
    }
}
