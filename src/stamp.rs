use chrono;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Stamp {
    utc: String,
    local: String
}
impl Stamp {
    pub fn new() -> Self {
        Stamp {
            utc: chrono::offset::Utc::now().to_string(),
            local: chrono::offset::Local::now().to_string()
        }
    }
}

