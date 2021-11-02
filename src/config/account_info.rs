use super::pool::PoolType;
use crate::{default_builder, default_builder_fn};
use rocket::serde::{Deserialize, Serialize};

pub fn new() -> AccountInfoBuilder {
    AccountInfoBuilder {
        user_name: String::from(""),
        user_token: String::from(""),
        user_access: vec![],
    }
}

default_builder!(
    #[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
    pub struct AccountInfo {
        pub user_name: String,
        pub user_token: String,
        pub user_access: Vec<PoolType>,
    },
    AccountInfoBuilder
);
