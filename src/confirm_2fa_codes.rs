use std::fmt;

use my_no_sql_server_abstractions::MyNoSqlEntity;
use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::*;

pub const ASSETS_TABLE_NAME: &str = "assets";

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Confirm2faCodeEntity {
    #[serde(rename = "PartitionKey")]
    pub code_type: String,
    #[serde(rename = "RowKey")]
    pub process_id: String,
    #[serde(rename = "TimeStamp")]
    pub time_stamp: String,
    #[serde(rename = "ClientId")]
    pub client_id: String,
    #[serde(rename = "Code")]
    pub code: i32,
}

impl Confirm2faCodeEntity {
    pub fn get_code_type(&self) -> Confirm2faCodeType {
        match self.code_type.as_str() {
            "Login" => Confirm2faCodeType::Login,
            "Withdrawal" => Confirm2faCodeType::Withdrawal,
            "Exchange" => Confirm2faCodeType::Exchange,
            "SettingsUpdate" => Confirm2faCodeType::SettingsUpdate,
            _ => panic!("Unknown code type {}", self.code_type),
        }
    }
}
