use rocket::serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Maven {
    pub source_url: String,
}

pub fn new(source_url: String) -> Maven {
    Maven { source_url }
}
