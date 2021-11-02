use rocket::{http::Status, local::blocking::Client};

use crate::services::{_host_state, api_state};

#[test]
fn test_get_host_state() {
    let rocket = rocket::build().mount("/api", routes![api_state::state]);
    let client = Client::tracked(rocket).unwrap();
    let req = client.get("/api/state").dispatch();

    assert_eq!(req.status(), Status::Ok);
}
