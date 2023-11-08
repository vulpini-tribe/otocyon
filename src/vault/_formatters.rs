use super::_types::{Connection, ConnectionFormattedList};

impl Connection {
    pub fn format_list(&self) -> ConnectionFormattedList {
        let formatted = ConnectionFormattedList {
            id: self.id.clone(),
        };

        formatted
    }
}
