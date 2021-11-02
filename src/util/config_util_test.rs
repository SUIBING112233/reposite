use std::io::{self};

use super::config_util;

#[test]
fn test_get_reposite_config_file() {
    match config_util::get_reposite_config_file() {
        Ok(r) => {
            println!("{:?}", &r)
        }
        Err(x) => {
            println!("{}", x);
        }
    }
}
