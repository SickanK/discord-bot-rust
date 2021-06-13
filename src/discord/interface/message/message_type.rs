use num::ToPrimitive;
#[derive(ToPrimitive, FromPrimitive, Debug)]
pub enum MessageType {
    Default = 0,
    RecipientAdd = 1,
    RecipientEmove = 2,
    Call = 3,
    ChannelNameChange = 4,
    ChannelIconChange = 5,
    ChannelPinnedMessage = 6,
    GuildMemberJoin = 7,
    UserPremiumGuildSubscription = 8,
    UserPremiumGuildSubscriptionTier1 = 9,
    UserPremiumGuildSubscriptionTier2 = 10,
    UserPremiumGuildSubscriptionTier3 = 11,
    ChannelFollowAdd = 12,
    GuildDiscoveryDisqualified = 14,
    GuildDiscoveryRequalified = 15,
    GuildDiscoveryGracePeriodInitialWarning = 16,
    GuildDiscoveryGracePeriodFinalWarning = 17,
    ThreadCreated = 18,
    Reply = 19,
    ApplicationCommand = 20,
    ThreadStarterMessage = 21,
    GuildInviteReminder = 22,
}
struct MessageTypeVisitor;

impl<'de> serde::de::Visitor<'de> for MessageTypeVisitor {
    type Value = MessageType;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("u16")
    }

    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match num::FromPrimitive::from_u16(v) {
            Some(f) => Ok(f),
            None => Err(E::custom("Failed to convert to MessageType")),
        }
    }
}

impl serde::ser::Serialize for MessageType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u16(self.to_u16().unwrap())
    }
}

impl<'de> serde::de::Deserialize<'de> for MessageType {
    fn deserialize<D>(deserializer: D) -> Result<MessageType, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_u16(MessageTypeVisitor)
    }
}
