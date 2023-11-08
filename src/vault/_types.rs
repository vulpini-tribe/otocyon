use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub auth_type: Option<String>,
    pub service_id: Option<String>,
    pub unified_api: Option<String>,
    pub icon: Option<String>,
    pub website: Option<String>,
    pub name: Option<String>,
    pub state: Option<String>,
    pub tag_line: Option<String>,
    pub authorize_url: Option<String>,
    pub revoke_url: Option<String>,
}
