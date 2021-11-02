use crate::config::rconfig;

use super::{config_manager, pool};

#[test]
fn test_config_manager() {
    assert_eq!(
        config_manager::get_config(),
        rconfig::new()
            .root_token(config_manager::get_config().root_token)
            .build()
    );
}

#[test]
fn test_add_pool() {
    let t_pool = pool::new().pool_type(pool::PoolType::None).build();
    config_manager::add_pool(t_pool.clone());
    let t_m = rconfig::new()
        .root_token(config_manager::get_config().root_token)
        .pools(vec![t_pool.clone()])
        .build();
    assert_eq!(t_m, config_manager::get_config());
}
