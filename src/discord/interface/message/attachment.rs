use crate::discord::snowflake::Snowflake;

#[derive(Serialize, Deserialize, Debug)]
pub struct Attachement {
    id: Snowflake,
    filename: String,
    content_type: Option<String>,
    size: usize,
    url: String,
    proxy_url: String,
    height: Option<usize>,
    width: Option<usize>,
}
