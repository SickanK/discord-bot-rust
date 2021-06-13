use num::ToPrimitive;
#[derive(ToPrimitive, FromPrimitive, Debug)]
pub enum PremiumType {
    None = 0,
    NitroClassic = 1,
    Nitro = 2,
}
struct PremiumTypeVisitor;

impl<'de> serde::de::Visitor<'de> for PremiumTypeVisitor {
    type Value = PremiumType;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("u16")
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match num::FromPrimitive::from_u16(v) {
            Some(f) => Ok(f),
            None => Err(E::custom("Failed to convert to PremiumType")),
        }
    }
}

impl serde::ser::Serialize for PremiumType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u16(self.to_u16().unwrap())
    }
}

impl<'de> serde::de::Deserialize<'de> for PremiumType {
    fn deserialize<D>(deserializer: D) -> Result<PremiumType, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_u16(PremiumTypeVisitor)
    }
}
