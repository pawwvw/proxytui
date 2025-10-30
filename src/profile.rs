use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Profile {
    pub name: String,
    pub uri: String,

    #[serde(skip)]
    pub connection: bool,
}

impl Profile {
    pub fn new(name: String, uri: String) -> Self {
        Self {
            name,
            uri,
            connection: false,
        }
    }
}
