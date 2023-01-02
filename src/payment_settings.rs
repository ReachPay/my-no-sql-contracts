use serde::{Deserialize, Serialize};

#[my_no_sql_macros::my_no_sql_entity("crypto-deposit-settings")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PaymentSettingsNoSql {
    #[serde(rename = "FireblockId")]
    pub fireblocks_id: String,
}

impl PaymentSettingsNoSql {
    pub fn new(asset: String, network: String, fireblocks_id: String) -> Self {
        Self {
            partition_key: asset,
            row_key: network,
            fireblocks_id,
            time_stamp: "".to_string(),
        }
    }
}
