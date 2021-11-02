use std::{
    fs::{self, File},
    io::{self, ErrorKind, Write},
};

use rocket::http::hyper::Uri;

use crate::{
    pool_cache::mvn_pool::{self, Maven},
    config::{pool, rconfig},
};

use super::path_util;

pub fn get_reposite_config_file() -> io::Result<File> {
    let x = path_util::get_working_path().join("reposite.json");
    match File::open(x.clone()) {
        // Exist.
        Ok(f) => Ok(f),
        // Not exist.
        Err(_) => match File::create(x.clone()) {
            Ok(file_open) => {
                match fs::write(x, serde_json::to_string(&build_default_config()).unwrap()) {
                    // Write succeed
                    Ok(_) => Ok(file_open),
                    // Write failed
                    Err(e) => Err(e),
                }
            }
            // Create failed.
            Err(e) => Err(e),
        },
    }
}

pub fn build_default_config() -> rconfig::RepositeConfig {
    rconfig::new()
        .pools(vec![pool::new()
            .pool_name("default_maven".to_string())
            .pool_type(pool::PoolType::Maven(vec![
                mvn_pool::new("https://repo1.maven.org/maven2/".to_string()),
                mvn_pool::new("https://jitpack.io/".to_string()),
            ]))
            .build()])
        .root_token("112233".to_string())
        .build()
}
