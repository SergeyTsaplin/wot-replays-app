use serde::{Deserialize, Serialize};
use std::vec::Vec;

#[derive(Deserialize, Serialize)]
pub struct WebError {
    pub code: u16,
    pub message: String,
}


#[derive(Deserialize, Serialize)]
pub struct ServerInfo {
    pub server_version: String,
    pub supported_api_versions: Vec<String>
}