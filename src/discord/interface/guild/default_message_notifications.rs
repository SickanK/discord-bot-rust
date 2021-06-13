use num::ToPrimitive;
#[derive(ToPrimitive, FromPrimitive, Debug)]
pub enum DefaultMessageNotifications {
    AllMessages = 0,
    OnlyMentions = 1,
}
struct DefaultMessageNotificationsVisitor;

impl<'de> serde::de::Visitor<'de> for DefaultMessageNotificationsVisitor {
    type Value = DefaultMessageNotifications;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("u16")
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match num::FromPrimitive::from_u16(v) {
            Some(f) => Ok(f),
            None => Err(E::custom(
                "Failed to convert to DefaultMessageNotifications",
            )),
        }
    }
}

impl serde::ser::Serialize for DefaultMessageNotifications {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u16(self.to_u16().unwrap())
    }
}

impl<'de> serde::de::Deserialize<'de> for DefaultMessageNotifications {
    fn deserialize<D>(deserializer: D) -> Result<DefaultMessageNotifications, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_u16(DefaultMessageNotificationsVisitor)
    }
}
