pub mod button_style;
pub mod message_component_type;

use crate::discord::interface::emoji::Emoji;

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageComponent {
    #[serde(rename = "type")]
    message_component_type: u8,
    style: Option<u8>,
    label: Option<String>,
    emoji: Option<Emoji>,
    custom_id: Option<String>,
    url: Option<String>,
    disabled: Option<bool>,
    components: Option<Vec<MessageComponent>>,
}
