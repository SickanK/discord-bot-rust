#[derive(Serialize, Deserialize, Debug)]
pub struct ActivitySecrets {
    join: Option<String>,
    spectate: Option<String>,
    #[serde(rename = "match")]
    activity_match: Option<String>,
}
