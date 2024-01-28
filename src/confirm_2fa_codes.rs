use std::fmt;
service_sdk::macros::use_my_no_sql_entity!();
use serde::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Confirm2faCodeType {
    Login,
    Withdrawal,
    Exchange,
    SettingsUpdate,
}

impl fmt::Display for Confirm2faCodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[my_no_sql_entity("assets")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Confirm2faCodeEntity {
    #[serde(rename = "ClientId")]
    pub client_id: String,
    #[serde(rename = "Code")]
    pub code: i32,
}

impl Confirm2faCodeEntity {
    pub fn get_code_type(&self) -> Confirm2faCodeType {
        match self.partition_key.as_str() {
            "Login" => Confirm2faCodeType::Login,
            "Withdrawal" => Confirm2faCodeType::Withdrawal,
            "Exchange" => Confirm2faCodeType::Exchange,
            "SettingsUpdate" => Confirm2faCodeType::SettingsUpdate,
            _ => panic!("Unknown code type {}", self.partition_key),
        }
    }

    pub fn get_process_id(&self) -> &str {
        &self.row_key
    }
}
