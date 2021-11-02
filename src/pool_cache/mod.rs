use std::{fs::File, path::PathBuf};

use log::debug;
use rocket::yansi::Paint;

use crate::{
    config::pool::PoolInfo,
    util::{
        self,
        download_mvn_util::{self, mvn_download},
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
                // Don't exist
                Err(_) => {
                    for index in x {
                        debug!(
                            "{} {}",
                            "target:",
                            Paint::white(req_path.clone().to_str().unwrap()).bold()
                        );
                        // Try get
                        match download_mvn_util::mvn_download(
                            url_util::pathbuf_url_helper(
                                index.source_url.clone(),
                                req_path.clone(),
                            ),
                            p.clone(),
                        )
                        .await
                        {
                            // Succeed.
                            Some(x) => {
                                debug!(
                                    "{} {}",
                                    "succeed to get target in ",
                                    Paint::green(index.source_url.clone()).italic()
                                );
                                return Some(x);
                            }
                            // Failed. And try next one.
                            None => {
                                debug!(
                                    "{} {}",
                                    "failed to get target in ",
                                    Paint::green(index.source_url).bold()
                                );
                                continue;
                            }
                        };
                    }
                }
            };
        }
    }

    None
}
