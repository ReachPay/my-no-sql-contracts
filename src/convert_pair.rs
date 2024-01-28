use serde::*;
service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("convert-paris")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConvertPairEntity {
    #[serde(rename = "FromSymbol")]
    pub from_symbol: String,
    #[serde(rename = "ToSymbol")]
    pub to_symbol: String,
    #[serde(rename = "IsEnabled")]
    pub is_enabled: bool,
}

impl ConvertPairEntity {
    pub fn generate_partition_key() -> &'static str {
        "asset-pairs"
    }

    pub fn get_asset_pair_id(&self) -> &str {
        &self.row_key
    }
}
