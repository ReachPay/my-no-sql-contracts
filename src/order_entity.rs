use serde::*;
service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("orders-to-pay")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderToPayMyNoSqlEntity {
    #[serde(rename = "Currency")]
    pub currency: String,

    #[serde(rename = "Amount")]
    pub amount: f64,

    #[serde(rename = "Registered")]
    pub registered: i64,

    #[serde(rename = "Expires")]
    pub expires: i64,

    #[serde(rename = "Commission")]
    pub commission: f64,

    #[serde(rename = "Memo")]
    pub memo: Option<String>,

    #[serde(rename = "IsCommissionOnTop")]
    pub is_commission_on_top: bool,
}

impl OrderToPayMyNoSqlEntity {
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

#[my_no_sql_entity("orders-to-be-payed")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderToBePaidMyNoSqlEntity {
    #[serde(rename = "Currency")]
    pub currency: String,

    #[serde(rename = "Amount")]
    pub amount: f64,

    #[serde(rename = "Registered")]
    pub registered: i64,

    #[serde(rename = "Expires")]
    pub expires: i64,

    #[serde(rename = "Commission")]
    pub commission: f64,

    #[serde(rename = "Memo")]
    pub memo: Option<String>,

    #[serde(rename = "IsCommissionOnTop")]
    pub is_commission_on_top: bool,
}

impl OrderToBePaidMyNoSqlEntity {
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
