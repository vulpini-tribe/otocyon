use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub id: String,
}

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ContactFormatted {
    pub id: String,
}

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ContactFormattedList {
    pub id: String,
}

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PostContact {
    pub title: String,
}

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct DeleteContacts {
    pub ids: Vec<String>,
}
