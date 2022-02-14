use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct WebError {
    pub code: u16,
    pub message: String,
}