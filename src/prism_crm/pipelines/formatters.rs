use super::pipeline_types::{Pipeline, PipelineFormatted, PipelineFormattedList};
use crate::types::Response;
use serde_json::Value;

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

pub fn to_pipeline(value: Response<Value>) -> Pipeline {
    let value = value.data.into();
    serde_json::from_value(value).unwrap()
}
