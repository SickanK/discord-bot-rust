use crate::discord::interface::emoji::Emoji;

#[derive(Serialize, Deserialize, Debug)]
pub struct Reaction {
    count: usize,
    me: bool,
    emoji: Emoji,
}
