use super::lead_types::{Lead, LeadFormatted, LeadFormattedList};
use crate::types::Response;
use serde_json::Value;

impl Lead {
    pub fn format_list(&self) -> LeadFormattedList {
        let formatted = LeadFormattedList {
            id: self.id.clone(),
        };

        formatted
    }

    pub fn format_one(&self) -> LeadFormatted {
        let formatted = LeadFormatted {
            id: self.id.clone(),
        };

        formatted
    }
}

pub fn to_lead(value: Response<Value>) -> Lead {
    let value = value.data.into();
    serde_json::from_value(value).unwrap()
}
