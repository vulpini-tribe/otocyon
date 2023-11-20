use serde::{Deserialize, Serialize};

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PipelineStage {
    pub id: String,
    pub name: String,
    pub value: Option<String>,
    pub win_probability: Option<i8>,
    pub display_order: Option<i32>,
}

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Pipeline {
    pub id: String,
    pub name: Option<String>,
    pub display_order: Option<i32>,
    pub archived: Option<bool>,
    pub currency: Option<String>,
    pub active: Option<bool>,
    pub win_probability_enabled: Option<bool>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,

    pub stages: Vec<PipelineStage>,
}

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct PostPipeline {
    pub title: String,
}

#[serde_with::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct DeletePipelines {
    pub ids: Vec<String>,
}
