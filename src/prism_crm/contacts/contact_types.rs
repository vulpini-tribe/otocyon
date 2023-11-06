use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ContactData {
    pub id: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub data: ContactData,
}
