use std::fmt::Display;

use super::{account_info::AccountInfo, pool::PoolInfo};
use crate::{config::pool, default_builder, default_builder_fn};
use rand::{distributions::Alphanumeric, Rng};
use rocket::serde::{Deserialize, Serialize};

pub fn new() -> RepositeConfigBuilder {
    RepositeConfigBuilder {
        root_token: rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(15)
            .map(char::from)
            .collect(),
        // user_token_info: vec![],
        pools: vec![pool::new().pool_name("test_pool".to_string()).build()],
    }
}

impl Display for RepositeConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

default_builder!(
    #[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
    pub struct RepositeConfig {
        pub root_token: String,
        // pub user_token_info: Vec<AccountInfo>,
        pub pools: Vec<PoolInfo>,
    },
    RepositeConfigBuilder
);
