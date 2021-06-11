use serde::{Deserialize, Serialize};

use super::emoji::Emoji;

#[derive(Serialize, Deserialize)]
pub struct Reaction {
    count: usize,
    me: bool,
    emoji: Emoji,
}
