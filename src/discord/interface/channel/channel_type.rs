use num::ToPrimitive;

#[derive(FromPrimitive, Debug, ToPrimitive)]
pub enum ChannelType {
    GuildText = 0,
    Dm = 1,
    GuildVoice = 2,
    GroupDm = 3,
    GuildCategory = 4,
    GuildNews = 5,
    GuildStore = 6,
    GuildNewsThread = 10,
    GuildPublicThread = 11,
    GuildPrivateThread = 12,
    GuildStageVoice = 13,
}

struct ChannelTypeVisitor;

impl<'de> serde::de::Visitor<'de> for ChannelTypeVisitor {
    type Value = ChannelType;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("u16")
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match num::FromPrimitive::from_u16(v) {
            Some(f) => Ok(f),
            None => Err(E::custom("Failed to convert to ChannelType")),
        }
    }
}

impl serde::ser::Serialize for ChannelType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u16(self.to_u16().unwrap())
    }
}

impl<'de> serde::de::Deserialize<'de> for ChannelType {
    fn deserialize<D>(deserializer: D) -> Result<ChannelType, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_u16(ChannelTypeVisitor)
    }
}
