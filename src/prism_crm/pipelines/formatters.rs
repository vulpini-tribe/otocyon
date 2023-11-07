use super::pipeline_types::{Pipeline, PipelineFormatted, PipelineFormattedList};

impl Pipeline {
    pub fn format_list(&self) -> PipelineFormattedList {
        let formatted = PipelineFormattedList {
            id: self.id.clone(),
        };

        formatted
    }

    pub fn format_one(&self) -> PipelineFormatted {
        let formatted = PipelineFormatted {
            id: self.id.clone(),
        };

        formatted
    }
}
