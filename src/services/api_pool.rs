use std::{
    fs::{self, File, FileType},
    io::Error,
    path::{Path, PathBuf},
};

use crate::{
    config::{config_manager, pool},
    util::path_util,
};
use rocket::{fairing::Result, form::validate::Len, http::ContentType};
use walkdir::WalkDir;

// "localhost/api/pool/"
#[get("/")]
pub async fn pool_list() -> (ContentType, String) {
    let x = config_manager::get_config().pools;
    (ContentType::JSON, serde_json::to_string(&x).unwrap())
}

// "localhost/api/pool/<pool_name>/<req_path..>"
#[get("/<pool_name>/<req_path..>")]
pub async fn path_info(
    pool_name: String,
    req_path: PathBuf,
) -> (ContentType, Option<String>) {
    match path_util::path_warden(pool_name, req_path) {
        Some(x) => (
            ContentType::JSON,
            Some(serde_json::to_string(&path_util::walk_path(x)).unwrap()),
        ),
        None => (ContentType::JSON, None),
    }
}
