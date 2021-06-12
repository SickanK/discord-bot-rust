use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Snowflake(String);

impl Snowflake {
    fn timestamp(&self) -> DateTime<Utc> {
        /*
        let timestamp = (&self.0 >> 22) + 1420070400000;
         */

        DateTime::from_utc(NaiveDateTime::from_timestamp(10, 0), Utc)
    }
}
