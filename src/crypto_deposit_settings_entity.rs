use serde::{Deserialize, Serialize};

#[my_no_sql_macros::my_no_sql_entity("crypto-deposit-settings")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]

pub struct CryptoDepositSettingsEntity {
    #[serde(rename = "FireblockId")]
    pub fireblocks_id: String,
    #[serde(rename = "BaseFireblocksAsset")]
    pub base_fireblocks_id: Option<String>,
    #[serde(rename = "RebalanceTrashhold")]
    pub rebalance_trashold: f64,
    #[serde(rename = "RebalanceGasFeeAmount")]
    pub rebalance_gas_fee_amount: f64,
}

impl CryptoDepositSettingsEntity {
    pub fn get_asset_id(&self) -> &str {
        &self.partition_key
    }

    pub fn get_network(&self) -> &str {
        &self.row_key
    }
}