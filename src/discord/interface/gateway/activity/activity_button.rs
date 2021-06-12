use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityButton {
    label: String,
    url: String,
}
