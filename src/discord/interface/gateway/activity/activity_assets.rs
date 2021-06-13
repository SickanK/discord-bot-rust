
#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityAssets {
    large_image: Option<String>,
    large_text: Option<String>,
    small_image: Option<String>,
    small_text: Option<String>,
}
