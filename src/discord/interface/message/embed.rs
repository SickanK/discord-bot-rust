use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Embed {
    title: Option<String>,

    #[serde(rename = "type")]
    embed_type: Option<String>,
    description: Option<String>,
    url: Option<String>,
    timestamp: DateTime<Utc>,
    color: usize,
    footer: Option<EmbedFooter>,
    image: Option<EmbedImage>,
    thumbnail: Option<EmbedThumbnail>,
    video: Option<EmbedVideo>,
    provider: Option<EmbedProvider>,
    author: Option<EmbedAuthor>,
    fields: Option<Vec<EmbedField>>,
}

#[derive(Serialize, Deserialize)]
pub struct EmbedFooter {
    text: String,
    icon_url: Option<String>,
    proxy_icon_url: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct EmbedImage {
    url: Option<String>,
    proxy_url: Option<String>,
    height: Option<usize>,
    width: Option<usize>,
}

#[derive(Serialize, Deserialize)]
pub struct EmbedThumbnail {
    url: Option<String>,
    proxy_url: Option<String>,
    height: Option<usize>,
    width: Option<usize>,
}

#[derive(Serialize, Deserialize)]
pub struct EmbedVideo {
    url: Option<String>,
    proxy_url: Option<String>,
    height: Option<usize>,
    width: Option<usize>,
}

#[derive(Serialize, Deserialize)]
pub struct EmbedProvider {
    name: Option<String>,
    url: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct EmbedAuthor {
    name: Option<String>,
    url: Option<String>,
    icon_url: Option<String>,
    proxy_icon_url: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct EmbedField {
    name: String,
    value: String,
    inline: Option<bool>,
}
