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
    format_type: u8,
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
        formatter.write_str("u16")
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match num::FromPrimitive::from_u16(v) {
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
        serializer.serialize_u16(self.to_u16().unwrap())
    }
}

impl<'de> serde::de::Deserialize<'de> for FormatType {
    fn deserialize<D>(deserializer: D) -> Result<FormatType, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_u16(FormatTypeVisitor)
    }
}
