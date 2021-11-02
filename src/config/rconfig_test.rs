use uuid::Uuid;

use crate::config::{
    self,
    account_info::AccountInfo,
    pool::{PoolInfo, PoolType},
    rconfig,
};

#[test]
fn test_reposite_config() {
    // Test data
    let t_user_name = String::from("test");
    let t_user_token = String::from("val");
    let t_user_access = vec![config::pool::PoolType::None];
    let t_uuid = Uuid::new_v4();

    let t_pool_info = PoolInfo {
        uuid: t_uuid.clone(),
        pool_type: PoolType::None,
        pool_name: "".to_string(),
    };
    let t_root_token = String::from("root_token");
    let t_user_info = AccountInfo {
        user_name: t_user_name.clone(),
        user_token: t_user_token.clone(),
        user_access: t_user_access,
    };

    // Instantiation via builder
    let t_b = rconfig::new()
        // .user_token_info(vec![t_user_info.clone()])
        .root_token(t_root_token.clone())
        .pools(vec![t_pool_info.clone()])
        .build();

    // Manual instantiation
    let t_m = rconfig::RepositeConfig {
        root_token: t_root_token.clone(),
        // user_token_info: vec![t_user_info.clone()],
        pools: vec![t_pool_info.clone()],
    };

    assert_eq!(t_b, t_m, "{} {}", t_b, t_m);
}
