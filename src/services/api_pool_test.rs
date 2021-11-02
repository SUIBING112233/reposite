use std::path::{Path, PathBuf};

use rocket::{http::Status, local::blocking::Client};

use crate::{services::api_pool, util::path_util};

#[test]
fn test_get_pool_list() {
    let rocket = rocket::build().mount("/pool", routes![api_pool::pool_list]);
    let client = Client::tracked(rocket).unwrap();
    let req = client.get("/pool/").dispatch();

    assert_eq!(req.status(), Status::Ok);
    println!("{}", req.into_string().unwrap());
}

#[test]
fn test_get_file_list() {
    let rocket = rocket::build().mount("/api", routes![api_pool::pool_list]);
    let client = Client::tracked(rocket).unwrap();

    // 1
    let req = client.get("/api/pool/test/tech/icedlab/advagri").dispatch();
    assert_eq!(req.status(), Status::NotFound);

    // 2
    let req = client.get("/api/pool/test/tech/../advagri").dispatch();
    assert_eq!(req.status(), Status::NotFound);
}
