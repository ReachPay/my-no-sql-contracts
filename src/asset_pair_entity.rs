use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::*;

pub const ASSET_PAIR_TABLE_NAME: &str = "asset-pairs";
#[my_no_sql_macros::my_no_sql_entity]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetPairEntity {
    #[serde(rename = "FromSymbol")]
    pub from_symbol: String,
    #[serde(rename = "ToSymbol")]
    pub to_symbol: String,
}

impl AssetPairEntity {
    pub fn generate_partition_key() -> &'static str {
        "asset-pairs"
    }

    pub fn get_asset_pair_id(&self) -> &str {
        &self.row_key
    }
}
