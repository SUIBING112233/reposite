use std::path::PathBuf;

use rocket::{
    http::ContentType,
    response::content::{self},
};

use crate::config::config_manager;

use super::_host_state;

// "localhost/api/state"
#[get("/state")]
pub fn state() -> (ContentType, String) {
    let mut x = _host_state::new();
    x.refresh();
    (ContentType::JSON, serde_json::to_string(&x).unwrap())
}
