use serde::*;
service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("price-bridges-config")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BridgeSettingsMyNoSqlEntity {
    #[serde(rename = "OutputTicker")]
    pub output_ticker: String,
}
