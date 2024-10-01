service_sdk::macros::use_my_no_sql_entity!();
use serde::*;

#[my_no_sql_entity("assets")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetEntity {
    #[serde(rename = "Accuracy")]
    pub accuracy: usize,
    #[serde(rename = "IsEnabled")]
    pub is_enabled: bool,
    #[serde(rename = "IsEnabledDeposit")]
    pub is_enabled_deposit: bool,
    #[serde(rename = "IsEnabledWithdrawal")]
    pub is_enabled_withdrawal: bool,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "IconUrl")]
    pub icon_url: String,
}

impl AssetEntity {
    pub fn generate_partition_key() -> &'static str {
        "assets"
    }

    pub fn get_asset_id(&self) -> &str {
        &self.row_key
    }
}
