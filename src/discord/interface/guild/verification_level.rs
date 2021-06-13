use num::ToPrimitive;
#[derive(ToPrimitive, FromPrimitive, Debug)]
pub enum VerificationLevel {
    None = 0,
    Low = 1,
    Medium = 2,
    High = 3,
    VeryHigh = 4,
}
struct VerificationLevelVisitor;

impl<'de> serde::de::Visitor<'de> for VerificationLevelVisitor {
    type Value = VerificationLevel;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("u16")
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match num::FromPrimitive::from_u16(v) {
            Some(f) => Ok(f),
            None => Err(E::custom("Failed to convert to VerificationLevel")),
        }
    }
}

impl serde::ser::Serialize for VerificationLevel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u16(self.to_u16().unwrap())
    }
}

impl<'de> serde::de::Deserialize<'de> for VerificationLevel {
    fn deserialize<D>(deserializer: D) -> Result<VerificationLevel, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_u16(VerificationLevelVisitor)
    }
}
