use num::ToPrimitive;
#[derive(ToPrimitive, FromPrimitive, Debug)]
pub enum VideoQualityMode {
    Auto = 1,
    Full = 2,
}

struct VideoQualityModeVisitor;

impl<'de> serde::de::Visitor<'de> for VideoQualityModeVisitor {
    type Value = VideoQualityMode;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("u16")
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match num::FromPrimitive::from_u16(v) {
            Some(f) => Ok(f),
            None => Err(E::custom("Failed to convert to VideoQualityMode")),
        }
    }
}

impl serde::ser::Serialize for VideoQualityMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u16(self.to_u16().unwrap())
    }
}

impl<'de> serde::de::Deserialize<'de> for VideoQualityMode {
    fn deserialize<D>(deserializer: D) -> Result<VideoQualityMode, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_u16(VideoQualityModeVisitor)
    }
}
