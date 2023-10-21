use crate::prism_crm::users::user_types;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Company {
    pub id: Option<String>,
    pub name: Option<String>,
    pub owner_id: Option<String>,
    pub owner: Option<user_types::CrmUser>,
}
