pub mod channel_type;

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageActivity {
    #[serde(rename = "type")]
    activity_type: u8,
    party_id: Option<String>,
}
