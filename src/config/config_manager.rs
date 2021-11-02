use std::sync::{Arc, Mutex};

use once_cell::sync::Lazy;
use uuid::Uuid;

use crate::util::config_util;

use super::{pool, rconfig};

static _CONFIG_INSTANCE: Lazy<Arc<Mutex<rconfig::RepositeConfig>>> = Lazy::new(|| {
    let f = config_util::get_reposite_config_file();
    match f {
        Ok(file) => match serde_json::from_reader::<_, rconfig::RepositeConfig>(file) {
            Ok(rc) => Arc::new(Mutex::new(rc)),
            Err(e) => {
                error!("{}", &e);
                warn!("use default config.");
                Arc::new(Mutex::new(config_util::build_default_config()))
            }
        },
        Err(e) => {
            error!("{}", e);
            warn!("use default config.");
            Arc::new(Mutex::new(config_util::build_default_config()))
        }
    }
});

pub fn save_config() {}

pub fn get_config() -> rconfig::RepositeConfig {
    _CONFIG_INSTANCE.lock().unwrap().clone()
}

pub fn is_pool_exist_uuid(uuid: Uuid) -> (Option<pool::PoolInfo>, usize) {
    let x = _CONFIG_INSTANCE.lock().unwrap().clone();
    for i in 0..x.pools.len() {
        if x.pools.get(i).unwrap().uuid == uuid {
            return (Some(x.pools.get(i).unwrap().clone()), i);
        }
    }
    return (None, 0);
}

pub fn is_pool_exist_pool_name(pool_name: String) -> (Option<pool::PoolInfo>, usize) {
    let x = _CONFIG_INSTANCE.lock().unwrap().clone();
    for i in 0..x.pools.len() {
        if x.pools.get(i).unwrap().pool_name == pool_name {
            return (Some(x.pools.get(i).unwrap().clone()), i);
        }
    }
    return (None, 0);
}

pub fn add_pool(pool: pool::PoolInfo) {
    let (x, _) = is_pool_exist_uuid(pool.uuid);
    match x {
        Some(_) => {}
        None => _CONFIG_INSTANCE.lock().unwrap().pools.push(pool),
    }
}

pub fn set_pool_by_poolinfo(pool: pool::PoolInfo) {
    let (x, y) = is_pool_exist_uuid(pool.uuid);
    match x {
        Some(_) => {
            _CONFIG_INSTANCE.lock().unwrap().pools.remove(y);
            _CONFIG_INSTANCE.lock().unwrap().pools.push(pool);
        }
        None => {}
    }
}

pub fn remove_pool_by_poolinfo(pool: pool::PoolInfo) {
    let (x, y) = is_pool_exist_uuid(pool.uuid);
    match x {
        Some(_) => {
            _CONFIG_INSTANCE.lock().unwrap().pools.remove(y);
        }
        None => {}
    }
}

pub fn remove_pool_by_uuid(uuid: uuid::Uuid) {
    let (x, y) = is_pool_exist_uuid(uuid);
    match x {
        Some(_) => {
            _CONFIG_INSTANCE.lock().unwrap().pools.remove(y);
        }
        None => {}
    }
}

pub fn remove_pool_by_index(i: usize) {
    let mut x = _CONFIG_INSTANCE.lock().unwrap();
    if i > x.pools.len() {
        return;
    }
    x.pools.remove(i);
}

pub fn remove_pool_by_pool_name(pool_name: String) {
    let (x, y) = is_pool_exist_pool_name(pool_name);
    match x {
        Some(_) => {
            _CONFIG_INSTANCE.lock().unwrap().pools.remove(y);
        }
        None => {}
    }
}

pub fn get_pool_by_uuid(uuid: uuid::Uuid) -> Option<usize> {
    let (x, y) = is_pool_exist_uuid(uuid);
    match x {
        Some(_) => Some(y),
        None => None,
    }
}

pub fn get_pool_by_pool_name(pool_name: String) -> Option<pool::PoolInfo> {
    let (x, _) = is_pool_exist_pool_name(pool_name);
    match x {
        Some(_) => x,
        None => None,
    }
}
