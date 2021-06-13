#[derive(Serialize, Deserialize)]
pub struct GatewayHeartbeat {
    pub op: u8,
    pub d: u32,
}

#[derive(Serialize, Deserialize)]
pub struct GatewayHello {
    pub op: u8,
    pub d: GatewayHelloData,
}

#[derive(Serialize, Deserialize)]
pub struct GatewayHelloData {
    pub heartbeat_interval: u64,
}

// Identify

#[derive(Serialize, Deserialize)]
pub struct GatewayIdentify {
    pub op: u8,
    pub d: GatewayIdentifyData,
}

#[derive(Serialize, Deserialize)]
pub struct GatewayIdentifyData {
    pub token: String,
    pub intents: u32,
    pub properties: GatewayIdentifyDataProperties,
}

#[derive(Serialize, Deserialize)]
pub struct GatewayIdentifyDataProperties {
    #[serde(rename = "$os")]
    pub os: String,

    #[serde(rename = "$browser")]
    pub browser: String,

    #[serde(rename = "$device")]
    pub device: String,
}
