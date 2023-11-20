use super::_types::{Note, NoteFormatted, NoteFormattedList};

impl Note {
    pub fn format_list(&self) -> NoteFormattedList {
        let formatted = NoteFormattedList {
            id: self.id.clone(),
            title: self.title.clone(),
            content: self.content.clone(),
            updated_at: self.updated_at.clone(),
            created_at: self.created_at.clone(),
        };

        formatted
    }

    pub fn format_one(&self) -> NoteFormatted {
        let formatted = NoteFormatted {
            id: self.id.clone(),
            title: self.title.clone().unwrap_or(String::from("")),
            content: self.content.clone().unwrap_or(String::from("")),
            active: self.active.clone().unwrap_or(true),
        };

        formatted
    }
}
