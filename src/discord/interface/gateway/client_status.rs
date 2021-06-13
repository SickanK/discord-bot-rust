#[derive(Serialize, Deserialize, Debug)]
pub struct ClientStatus {
    desktop: Option<String>,
    mobile: Option<String>,
    web: Option<String>,
}
