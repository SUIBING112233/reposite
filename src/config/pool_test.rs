use uuid::Uuid;

use super::pool;

#[test]
fn test_pool_info() {
    let t_uuid = Uuid::new_v4();
    let t_b_1 = pool::new()
        .pool_type(pool::PoolType::None)
        .uuid(t_uuid.clone())
        .build();

    let t_m_1 = pool::PoolInfo {
        pool_name: "".to_string(),
        uuid: t_uuid.clone(),
        pool_type: pool::PoolType::None,
    };
    assert_eq!(t_b_1, t_m_1);

    let t_b_2 = pool::new().pool_type(pool::PoolType::Maven(vec![])).build();
    let t_m_2 = pool::PoolInfo {
        pool_name: "".to_string(),
        uuid: Uuid::new_v4(),
        pool_type: pool::PoolType::None,
    };
    assert_ne!(t_b_2, t_m_2);
}
