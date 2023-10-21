use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
    pub data: Option<T>,
    pub status_code: Option<u16>,
    pub status: Option<String>,
}
