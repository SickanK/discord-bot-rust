use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, FromPrimitive, Debug)]
pub enum MFALevel {
    None = 0,
    Elevated = 1,
}
