use crate::types::{Address, PhoneNumber};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Lead {
    pub id: String,
}
