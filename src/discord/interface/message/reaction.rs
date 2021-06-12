use serde::{Deserialize, Serialize};

use crate::discord::interface::emoji::Emoji;

#[derive(Serialize, Deserialize)]
pub struct Reaction {
    count: usize,
    me: bool,
    emoji: Emoji,
}
