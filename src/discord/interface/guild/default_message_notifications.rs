use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, FromPrimitive, Debug)]
pub enum DefaultMessageNotifications {
    AllMessages = 0,
    OnlyMentions = 1,
}
