use std::path::PathBuf;

use log::{debug, info};
use rocket::{fs::NamedFile, http::ContentType, yansi::Paint};

use crate::{config::config_manager, pool_cache, util::path_util};

// "localhost/pool/<pool_name>/<path..>"
#[get("/<pool_name>/<path..>")]
pub async fn get_file(pool_name: String, path: PathBuf) -> (ContentType, Option<NamedFile>) {
    debug!(
        "{} {}",
        "request pool:",
        Paint::default(pool_name.clone()).bold()
    );
    debug!(
        "{} {}",
        "request target:",
        Paint::default(path.clone().to_str().unwrap()).bold()
    );

    match path_util::path_warden(pool_name.clone(), path.clone()) {
        Some(_) => {}
        None => return (ContentType::Binary, None),
    }

    match config_manager::get_pool_by_pool_name(pool_name.clone()) {
        Some(x) => match pool_cache::get(x, path).await {
            Some(x) => (ContentType::Binary, NamedFile::open(x).await.ok()),
            None => (ContentType::Binary, None),
        },
        None => (ContentType::Binary, None),
    }
}
