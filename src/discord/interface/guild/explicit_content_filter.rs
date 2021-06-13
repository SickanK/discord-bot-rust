use num::ToPrimitive;
#[derive(ToPrimitive, FromPrimitive, Debug)]
pub enum ExplicitContentFilter {
    Disabled = 0,
    MembersWithoutRoles = 1,
    AllMembers = 2,
}

struct ExplicitContentFilterVisitor;

impl<'de> serde::de::Visitor<'de> for ExplicitContentFilterVisitor {
    type Value = ExplicitContentFilter;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("u16")
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match num::FromPrimitive::from_u16(v) {
            Some(f) => Ok(f),
            None => Err(E::custom("Failed to convert to ExplicitContentFilter")),
        }
    }
}

impl serde::ser::Serialize for ExplicitContentFilter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u16(self.to_u16().unwrap())
    }
}

impl<'de> serde::de::Deserialize<'de> for ExplicitContentFilter {
    fn deserialize<D>(deserializer: D) -> Result<ExplicitContentFilter, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_u16(ExplicitContentFilterVisitor)
    }
}
