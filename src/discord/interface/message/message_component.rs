use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};

use super::emoji::Emoji;
#[derive(Serialize, Deserialize)]
pub struct MessageComponent {
    #[serde(rename = "type")]
    message_component_type: MessageComponentType,
    style: Option<ButtonStyle>,
    label: Option<String>,
    emoji: Option<Emoji>,
    custom_id: Option<String>,
    url: Option<String>,
    disabled: Option<bool>,
    components: Option<Vec<MessageComponent>>,
}

#[derive(FromPrimitive, Serialize, Deserialize)]
pub enum MessageComponentType {
    ActionRow = 1,
    Button = 2,
}

#[derive(FromPrimitive, Serialize, Deserialize)]
pub enum ButtonStyle {
    Primary = 1,
    Secondary = 2,
    Success = 3,
    Danger = 4,
    Link = 5,
}
