use std::collections::HashMap;
use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("instrument-mapping")]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstrumentMappingEntity {
    #[serde(rename = "LpId")]
    pub liquidity_provider_id: String,
    #[serde(rename = "Map")]
    pub map: HashMap<String, String>,
}