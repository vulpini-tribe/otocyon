use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct LeadData {
    pub id: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Lead {
    pub data: LeadData,
}
