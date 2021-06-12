use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize, Serializer};

#[derive(Serialize, Deserialize, FromPrimitive, Debug)]
pub enum PremiumTier {
    None = 0,
    Tier1 = 1,
    Tier2 = 2,
    Tier3 = 3,
}
