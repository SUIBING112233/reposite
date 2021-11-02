use crate::default_builder;
use crate::default_builder_fn;
use crate::pool_cache;
use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum PoolType {
    None,
    Maven(Vec<pool_cache::mvn_pool::Maven>),
}

pub fn new() -> PoolInfoBuilder {
    PoolInfoBuilder {
        uuid: Uuid::new_v4(),
        pool_type: PoolType::None,
        pool_name: "".to_string(),
    }
}

default_builder!(
    #[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
    pub struct PoolInfo {
        pub pool_name: String,
        pub uuid: uuid::Uuid,
        pub pool_type: PoolType,
    },
    PoolInfoBuilder
);
