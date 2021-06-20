use crate::discord::snowflake::Snowflake;
use num::ToPrimitive;
#[derive(Serialize, Deserialize, Debug)]
pub struct Sticker {
    id: Snowflake,
    pack_id: Snowflake,
    name: String,
    description: String,
    tags: Option<String>,
    asset: String,
    format_type: FormatType,
}

#[derive(ToPrimitive, FromPrimitive)]
pub enum FormatType {
    Png = 1,
    APng = 2,
    Lottie = 3,
}
struct FormatTypeVisitor;

impl<'de> serde::de::Visitor<'de> for FormatTypeVisitor {
    type Value = FormatType;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("u64")
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match num::FromPrimitive::from_u64(v) {
            Some(f) => Ok(f),
            None => Err(E::custom("Failed to convert to FormatType")),
        }
    }
}

impl serde::ser::Serialize for FormatType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u64(self.to_u64().unwrap())
    }
}

impl<'de> serde::de::Deserialize<'de> for FormatType {
    fn deserialize<D>(deserializer: D) -> Result<FormatType, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_u64(FormatTypeVisitor)
    }
}
