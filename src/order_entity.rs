use my_no_sql_server_abstractions::MyNoSqlEntity;
use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::*;

pub const ORDERS_TO_PAY_TABLE_NAME: &str = "orders-to-pay";
pub const ORDERS_TO_BE_PAIED_TABLE_NAME: &str = "orders-to-be-paied";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderMyNoSqlEntity {
    #[serde(rename = "PartitionKey")]
    pub partition_key: String,

    #[serde(rename = "RowKey")]
    pub row_key: String,

    #[serde(rename = "TimeStamp")]
    pub time_stamp: String,

    #[serde(rename = "Currency")]
    pub currency: String,

    #[serde(rename = "Amount")]
    pub amount: f64,

    #[serde(rename = "Registered")]
    pub registered: i64,

    #[serde(rename = "Expires")]
    pub expires: i64,

    #[serde(rename = "Comission")]
    pub comission: f64,

    #[serde(rename = "PaymentAddress")]
    pub payment_address: Option<String>,
}

impl OrderMyNoSqlEntity {
    pub fn generate_partition_key(user_id: &str) -> &str {
        user_id
    }

    pub fn generate_row_key(order_id: &str) -> &str {
        order_id
    }

    pub fn get_user_id(&self) -> &str {
        self.partition_key.as_str()
    }

    pub fn get_order_id(&self) -> &str {
        self.row_key.as_str()
    }
}

impl MyNoSqlEntity for OrderMyNoSqlEntity {
    fn get_partition_key(&self) -> &str {
        &self.partition_key
    }

    fn get_row_key(&self) -> &str {
        &self.row_key
    }

    fn get_time_stamp(&self) -> i64 {
        DateTimeAsMicroseconds::parse_iso_string(self.time_stamp.as_str())
            .unwrap()
            .unix_microseconds
    }
}
