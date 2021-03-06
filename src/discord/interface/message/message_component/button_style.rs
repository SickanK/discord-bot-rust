use num::ToPrimitive;
#[derive(ToPrimitive, FromPrimitive)]
pub enum ButtonStyle {
    Primary = 1,
    Secondary = 2,
    Success = 3,
    Danger = 4,
    Link = 5,
}
struct ButtonStyleVisitor;

impl<'de> serde::de::Visitor<'de> for ButtonStyleVisitor {
    type Value = ButtonStyle;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("u64")
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match num::FromPrimitive::from_u64(v) {
            Some(f) => Ok(f),
            None => Err(E::custom("Failed to convert to ButtonStyle")),
        }
    }
}

impl serde::ser::Serialize for ButtonStyle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u64(self.to_u64().unwrap())
    }
}

impl<'de> serde::de::Deserialize<'de> for ButtonStyle {
    fn deserialize<D>(deserializer: D) -> Result<ButtonStyle, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_u64(ButtonStyleVisitor)
    }
}
