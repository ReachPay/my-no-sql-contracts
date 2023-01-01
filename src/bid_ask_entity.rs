use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::*;

#[my_no_sql_macros::my_no_sql_entity("bid-ask-snapshot")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BidAskMyNoSqlEntity {
    #[serde(rename = "Bid")]
    pub bid: f64,
    #[serde(rename = "Ask")]
    pub ask: f64,
    #[serde(rename = "DateTime")]
    pub date_time: String,
}

impl BidAskMyNoSqlEntity {
    pub fn new(id: String, bid: f64, ask: f64, date_time: DateTimeAsMicroseconds) -> Self {
        let date_time = date_time.to_rfc3339();
        BidAskMyNoSqlEntity {
            partition_key: Self::get_partition_key().to_string(),
            row_key: id,
            time_stamp: date_time.clone(),
            bid,
            ask,
            date_time,
        }
    }

    pub fn get_partition_key() -> &'static str {
        "bidask"
    }

    pub fn get_date_time(&self) -> DateTimeAsMicroseconds {
        DateTimeAsMicroseconds::parse_iso_string(self.date_time.as_str()).unwrap()
    }
}
