use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, FromPrimitive, Debug)]
pub enum VideoQualityMode {
    Auto = 1,
    Full = 2,
}
