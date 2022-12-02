use my_no_sql_server_abstractions::MyNoSqlEntity;
use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::*;

pub const ASSETS_TABLE_NAME: &str = "assets";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MerchantCallbackSettingsOverride {
    #[serde(rename = "PartitionKey")]
    pub merchant_id: String,
    //instrument@network_id
    #[serde(rename = "RowKey")]
    pub instrument_network_id: String,
    #[serde(rename = "TimeStamp")]
    pub time_stamp: String,
    #[serde(rename = "CallbackType")]
    pub callback_type: i32,
}

impl MerchantCallbackSettingsOverride {
    pub fn generate_partition_key(&self) -> &str {
        &self.merchant_id
    }

    pub fn get_asset_id(&self) -> &str {
        &self.instrument_network_id
    }

    pub fn split_currency_and_network(&self) -> (&str, &str) {
        let parts: Vec<&str> = self.instrument_network_id.split("@").collect();
        return (parts[0], parts[1])
    }
}

impl MyNoSqlEntity for MerchantCallbackSettingsOverride {
    fn get_partition_key(&self) -> &str {
        &self.merchant_id
    }

    fn get_row_key(&self) -> &str {
        &self.instrument_network_id
    }

    fn get_time_stamp(&self) -> i64 {
        DateTimeAsMicroseconds::parse_iso_string(self.time_stamp.as_str())
            .unwrap()
            .unix_microseconds
    }
}
