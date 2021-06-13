use chrono::{DateTime, NaiveDateTime, Utc};

#[derive(Serialize, Deserialize, Debug)]
pub struct Snowflake(String);

impl Snowflake {
    fn timestamp(&self) -> DateTime<Utc> {
        let timestamp = (&self.0.parse::<i64>().unwrap() >> 22) + 1420070400000;

        DateTime::from_utc(NaiveDateTime::from_timestamp(timestamp, 0), Utc)
    }
}
