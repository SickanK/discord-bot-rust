use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ActivityTimestamp {
    start: Option<usize>,
    end: Option<usize>,
}
