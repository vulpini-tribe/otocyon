use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PipelineData {
    pub id: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Pipeline {
    pub data: PipelineData,
}
