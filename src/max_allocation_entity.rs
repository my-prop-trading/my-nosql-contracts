use rust_extensions::StrOrString;
use serde::*;

service_sdk::macros::use_my_no_sql_entity!();

#[my_no_sql_entity("max-allocation-by-volume")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct MaxAllocationByVolumeMyNoSqlEntity {
    pub max_allocation_volume: f64,
    pub is_enabled: bool,
}

impl MaxAllocationByVolumeMyNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "volumes"
    }

    pub fn generate_row_key() -> &'static str {
        "c"
    }
}

#[my_no_sql_entity("max-allocation-by-number")]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct MaxAllocationByNumberMyNoSqlEntity {
    pub max_allocation_number: i32,
    pub is_enabled: bool,
}

impl MaxAllocationByNumberMyNoSqlEntity {
    pub fn generate_partition_key() -> &'static str {
        "numbers"
    }

    pub fn generate_row_key() -> &'static str {
        "c"
    }
}
