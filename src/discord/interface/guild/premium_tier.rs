use num::ToPrimitive;
#[derive(ToPrimitive, FromPrimitive, Debug)]
pub enum PremiumTier {
    None = 0,
    Tier1 = 1,
    Tier2 = 2,
    Tier3 = 3,
}

struct PremiumTierVisitor;

impl<'de> serde::de::Visitor<'de> for PremiumTierVisitor {
    type Value = PremiumTier;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("u16")
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match num::FromPrimitive::from_u16(v) {
            Some(f) => Ok(f),
            None => Err(E::custom("Failed to convert to PremiumTier")),
        }
    }
}

impl serde::ser::Serialize for PremiumTier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u16(self.to_u16().unwrap())
    }
}

impl<'de> serde::de::Deserialize<'de> for PremiumTier {
    fn deserialize<D>(deserializer: D) -> Result<PremiumTier, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_u16(PremiumTierVisitor)
    }
}
