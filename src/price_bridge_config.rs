use serde::*;
service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("price-bridges-config")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PriceBridgeConfigMyNoSqlEntity {
    #[serde(rename = "ConnectionString")]
    pub connection_string: String,

    #[serde(rename = "IsEnabled")]
    pub is_enabled: bool,
}

impl PriceBridgeConfigMyNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "price-bridges-config"
    }

    pub fn get_bridge_id(&self) -> &str {
        self.row_key.as_str()
    }
}
