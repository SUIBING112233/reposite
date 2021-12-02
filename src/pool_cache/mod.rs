use std::{fs::File, path::PathBuf};

use log::debug;
use rocket::yansi::Paint;

use crate::{
    config::pool::PoolInfo,
    util::{
        self,
        download_mvn_util::{self, mvn_download_from_all_sources, mvn_download_from_source},
        path_util, url_util,
    },
};

pub mod mvn_pool;

pub trait CachePath {
    fn get_cache(self) -> Option<PathBuf>;
}

pub async fn get(pool_info: PoolInfo, req_path: PathBuf) -> Option<PathBuf> {
    match pool_info.clone().pool_type {
        crate::config::pool::PoolType::None => return None,
        crate::config::pool::PoolType::Maven(x) => {
            let p = PathBuf::from(path_util::get_working_path())
                .join("pool")
                .join(pool_info.pool_name.clone())
                .join(req_path.clone());

            match File::open(p.clone()) {
                // If exist
                Ok(_) => {
                    debug!(
                        "{} {}",
                        "cache found:",
                        Paint::green(req_path.clone().to_str().unwrap()).bold()
                    );

                    return Some(p);
                }
                // Don't exist. Download from sources
                Err(_) => {
                    debug!(
                        "{} {}",
                        "cache not found:",
                        Paint::red(req_path.clone().to_str().unwrap()).bold(),
                    );
                    match mvn_download_from_all_sources(x, req_path.clone()).await {
                        Some(x) => {
                            debug!(
                                "{} {}",
                                "downloaded:",
                                Paint::blue(req_path.clone().to_str().unwrap()).bold()
                            );
                            Some(x)
                        }
                        None => None,
                    }
                }
            };
        }
    }

    None
}
