use my_no_sql_server_abstractions::MyNoSqlEntity;
use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::{Deserialize, Serialize};

pub const TABLE_NAME: &str = "crypto-deposit-settings";

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CryptoDepositSettingsEntity {
    #[serde(rename = "PartitionKey")]
    pub asset: String,
    #[serde(rename = "RowKey")]
    pub network: String,
    #[serde(rename = "FireblockId")]
    pub fireblocks_id: String,
    #[serde(rename = "TimeStamp")]
    pub time_stamp: String,
}

impl MyNoSqlEntity for CryptoDepositSettingsEntity {
    fn get_partition_key(&self) -> &str {
        self.asset.as_str()
    }

    fn get_row_key(&self) -> &str {
        &self.network
    }

    fn get_time_stamp(&self) -> i64 {
        DateTimeAsMicroseconds::parse_iso_string(self.time_stamp.as_str())
            .unwrap()
            .unix_microseconds
    }
}
