use my_no_sql_server_abstractions::MyNoSqlEntity;
use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::*;

pub const ASSETS_TABLE_NAME: &str = "assets";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetEntity {
    #[serde(rename = "PartitionKey")]
    pub partition_key: String,
    #[serde(rename = "RowKey")]
    pub row_key: String,
    #[serde(rename = "TimeStamp")]
    pub time_stamp: String,
    #[serde(rename = "Accuracy")]
    pub accuracy: usize,
    #[serde(rename = "IsEnabled")]
    pub is_enabled: bool,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Id")]
    pub id: String,
}

impl AssetEntity {
    pub fn generate_partition_key() -> &'static str {
        "assets"
    }

    pub fn get_asset_id(&self) -> &str {
        &self.row_key
    }
}

impl MyNoSqlEntity for AssetEntity {
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
