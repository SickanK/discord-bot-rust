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
        formatter.write_str("u64")
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match num::FromPrimitive::from_u64(v) {
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
        serializer.serialize_u64(self.to_u64().unwrap())
    }
}

impl<'de> serde::de::Deserialize<'de> for VerificationLevel {
    fn deserialize<D>(deserializer: D) -> Result<VerificationLevel, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_u64(VerificationLevelVisitor)
    }
}
