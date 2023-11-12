use super::_types::{Activity, ActivityFormatted, ActivityFormattedList};

impl Activity {
    pub fn format_list(&self) -> ActivityFormattedList {
        let formatted = ActivityFormattedList {
            id: self.id.clone(),
            r#type: self.r#type.clone(),
            title: self.title.clone().unwrap_or(String::from("")),
            updated_at: self.updated_at.clone(),
            description: self.description.clone(),
        };

        formatted
    }

    pub fn format_one(&self) -> ActivityFormatted {
        let formatted = ActivityFormatted {
            id: self.id.clone(),
            r#type: self.r#type.clone(),
            title: self.title.clone().unwrap_or(String::from("")),
            updated_at: self.updated_at.clone(),
            description: self.description.clone(),
        };

        formatted
    }
}
