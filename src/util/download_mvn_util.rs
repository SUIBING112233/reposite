use std::{fs, io::ErrorKind, path::PathBuf};

use reqwest::Url;
use rocket::{http::Status, yansi::Paint};

use crate::{
    config::pool::PoolType,
    pool_cache::mvn_pool::Maven,
    util::{download_mvn_util, url_util},
};

pub async fn mvn_download_from_source(download_url: Url, storage: PathBuf) -> Option<PathBuf> {
    let x = reqwest::get(download_url.clone()).await.unwrap();
    if x.status().is_success() {
        let mut dir = storage.clone();
        dir.pop();
        match fs::create_dir_all(dir) {
            Ok(_) => {}
            Err(e) => {
                if e.kind() != ErrorKind::AlreadyExists {
                    return None;
                }
            }
        };

        // TODO!
        // Here need cache checking!
        //
        // If cache is too old, update it.
        //
        // Outdated time is 10s.

        // TODO!
        // Here need maven-metadata.xml checking!
        //
        // Because there maybe some cache time back!

        let file = storage.clone();

        // Succeed. Write it into cache pool.
        match fs::write(file, x.bytes().await.unwrap()) {
            Ok(_) => Some(storage.clone()),
            Err(_) => {
                return None;
            }
        };
    }

    None
}

pub async fn mvn_download_from_all_sources(maven: Vec<Maven>, storage: PathBuf) -> Option<PathBuf> {
    for index in maven {
        let d = mvn_download_from_source(
            url_util::pathbuf_url_helper(index.source_url, storage.clone()),
            storage.clone(),
        )
        .await;

        match d {
            Some(x) => return Some(x),
            None => continue,
        }
    }
    None
}
