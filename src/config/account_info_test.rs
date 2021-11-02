use crate::config::account_info;

#[test]
fn test_account_info() {
    let t_b = account_info::new().build();
    let t_m = account_info::AccountInfo {
        user_name: "".to_string(),
        user_token: "".to_string(),
        user_access: vec![],
    };
    assert_eq!(t_b, t_m);
}
