use num::ToPrimitive;
#[derive(ToPrimitive, FromPrimitive, Debug)]
pub enum NSFWLevel {
    Default = 0,
    Explicit = 1,
    Safe = 2,
    AgeRestricted = 3,
}
struct NSFWLevelVisitor;

impl<'de> serde::de::Visitor<'de> for NSFWLevelVisitor {
    type Value = NSFWLevel;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("u64")
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match num::FromPrimitive::from_u64(v) {
            Some(f) => Ok(f),
            None => Err(E::custom("Failed to convert to NSFWLevel")),
        }
    }
}

impl serde::ser::Serialize for NSFWLevel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u64(self.to_u64().unwrap())
    }
}

impl<'de> serde::de::Deserialize<'de> for NSFWLevel {
    fn deserialize<D>(deserializer: D) -> Result<NSFWLevel, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_u64(NSFWLevelVisitor)
    }
}
