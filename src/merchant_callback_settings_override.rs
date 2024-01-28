use serde::*;
service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("merchant-callback-settings-override")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MerchantCallbackSettingsOverride {
    #[serde(rename = "CallbackType")]
    pub callback_type: i32,
}

impl MerchantCallbackSettingsOverride {
    pub fn new(client_id: String, currency: &str, network: &str, callback_type: i32) -> Self {
        Self {
            partition_key: client_id,
            row_key: format!("{}@{}", currency, network),
            callback_type,
            time_stamp: "".to_string(),
        }
    }

    pub fn generate_merchant_id(&self) -> &str {
        &self.partition_key
    }

    pub fn get_instrument_network_id(&self) -> &str {
        &self.row_key
    }

    pub fn split_currency_and_network(&self) -> (&str, &str) {
        let parts: Vec<&str> = self.get_instrument_network_id().split("@").collect();
        return (parts[0], parts[1]);
    }
}
