use my_no_sql_server_abstractions::MyNoSqlEntity;
use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::*;

pub const PARTITION_KEY: &str = "price-bridges-config";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PriceBridgeConfigMyNoSqlEntity {
    #[serde(rename = "PartitionKey")]
    pub partition_key: String,

    #[serde(rename = "RowKey")]
    pub row_key: String,

    #[serde(rename = "TimeStamp")]
    pub time_stamp: String,

    #[serde(rename = "ConnectionString")]
    pub connection_string: String,

    #[serde(rename = "IsEnabled")]
    pub is_enabled: bool,
}

impl PriceBridgeConfigMyNoSqlEntity {
    pub fn get_bridge_id(&self) -> &str {
        self.row_key.as_str()
    }
}

impl MyNoSqlEntity for PriceBridgeConfigMyNoSqlEntity {
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
