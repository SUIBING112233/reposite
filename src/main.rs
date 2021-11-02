// #![feature(trace_macros)]
#![feature(async_closure)]

mod config;
mod constants;
mod macros;
mod pool_cache;
mod services;
mod task;
mod util;

use crate::config::config_manager;
use crate::services::api_pool;
use crate::services::api_state;
use crate::services::pool_service;
use crate::util::path_util;
use log::debug;
use rocket::Build;
use rocket::Rocket;
use simple_logger::SimpleLogger;

#[macro_use]
extern crate log;
#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> Rocket<Build> {
    SimpleLogger::new().init().unwrap();

    debug!("storage path:{:?}", path_util::get_working_path());
    debug!("pools:{:?}", {
        let mut x: Vec<String> = vec![];
        for i in config_manager::get_config().pools {
            x.push(i.pool_name);
        }
        x
    });

    rocket::build()
        .mount("/api", routes![api_state::state])
        .mount(
            "/api/pool",
            routes![api_pool::path_info, api_pool::pool_list],
        )
        .mount("/pool", routes![pool_service::get_file])
}
