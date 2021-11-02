use rocket::futures::task::SpawnExt;
use tokio::runtime;

use crate::services::_host_state;

#[test]
fn test_async_host_state_service() {
    let a = _host_state::new();
    assert_eq!(a, _host_state::new());
}
