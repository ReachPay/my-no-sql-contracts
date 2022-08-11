use my_no_sql_server_abstractions::MyNoSqlEntity;
use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::*;

pub const BID_ASK_SNAPSHOT_TABLE_NAME: &str = "bid-ask-snapshot";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BidAskMyNoSqlEntity {
    #[serde(rename = "PartitionKey")]
    pub partition_key: String,
    #[serde(rename = "RowKey")]
    pub row_key: String,
    #[serde(rename = "TimeStamp")]
    pub time_stamp: String,

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
            partition_key: "bidask".to_string(),
            row_key: id,
            time_stamp: date_time.clone(),
            bid,
            ask,
            date_time,
        }
    }
}

impl MyNoSqlEntity for BidAskMyNoSqlEntity {
    fn get_partition_key(&self) -> &str {
        &self.partition_key
    }

    fn get_row_key(&self) -> &str {
        &self.row_key
    }

    fn get_time_stamp(&self) -> i64 {
        DateTimeAsMicroseconds::parse_iso_string(self.time_stamp.as_str())
            .unwrap()
            .unix_microseconds
    }
}
