use my_no_sql_server_abstractions::MyNoSqlEntity;
use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::*;

pub const ASSET_PAIR_TABLE_NAME: &str = "asset-pairs";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetPairEntity {
    #[serde(rename = "PartitionKey")]
    pub partition_key: String,
    #[serde(rename = "RowKey")]
    pub row_key: String,
    #[serde(rename = "TimeStamp")]
    pub time_stamp: String,
    #[serde(rename = "FromSymbol")]
    pub from_symbol: String,
    #[serde(rename = "ToSymbol")]
    pub to_symbol: String,
    #[serde(rename = "IsEnabled")]
    pub is_enabled: bool,
}

impl AssetPairEntity {
    pub fn generate_partition_key() -> &'static str {
        "asset-pairs"
    }

    pub fn get_asset_pair_id(&self) -> &str {
        &self.row_key
    }
}

impl MyNoSqlEntity for AssetPairEntity {
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
