#[derive(Serialize, Deserialize, EnumString, Debug)]
pub enum GuildFeature {
    #[strum(serialize = "ANIMATED_ICON")]
    AnimatedIcon,
    #[strum(serialize = "BANNER")]
    Banner,
    #[strum(serialize = "COMMERCE")]
    Commerce,
    #[strum(serialize = "COMMUNITY")]
    Community,
    #[strum(serialize = "DISCOVERABLE")]
    Discoverable,
    #[strum(serialize = "FEATURABLE")]
    Featurable,
    #[strum(serialize = "INVITE_SPLASH")]
    InviteSplash,
    #[strum(serialize = "MEMBER_VERIFICATION_GATE_ENABLED")]
    MemberVerificationGateEnabled,
    #[strum(serialize = "NEWS")]
    News,
    #[strum(serialize = "PARTNERED")]
    Partnered,
    #[strum(serialize = "PREVIEW_ENABLED")]
    PreviewEnabled,
    #[strum(serialize = "VANITY_URL")]
    VanityUrl,
    #[strum(serialize = "VERIFIED")]
    Verified,
    #[strum(serialize = "VIP_REGIONS")]
    VipRegions,
    #[strum(serialize = "WELCOME_SCREEN_ENABLED")]
    WelcomeScreenEnabled,
}
