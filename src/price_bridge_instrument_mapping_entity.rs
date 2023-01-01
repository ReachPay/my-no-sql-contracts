use serde::*;

#[my_no_sql_macros::my_no_sql_entity("price-bridges-instruments-mapping")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PriceBridgeInstrumentMappingMyNoSqlEntity {}

impl PriceBridgeInstrumentMappingMyNoSqlEntity {
    pub fn is_my_instrument(&self, bridge_id: &str, instrument_id: &str) -> bool {
        self.partition_key == bridge_id && self.row_key == instrument_id
    }
}
