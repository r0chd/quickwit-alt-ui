#[derive(serde::Deserialize, Debug, Clone, PartialEq)]
pub struct Index {
    pub index_config: IndexConfig,
    pub create_timestamp: i64,
    pub sources: Vec<Source>,
}

#[derive(serde::Deserialize, Debug, Clone, PartialEq)]
pub struct IndexConfig {
    pub index_id: String,
    pub index_uri: String,
    pub doc_mapping: DocMapping,
}

#[derive(serde::Deserialize, Debug, Clone, PartialEq)]
pub struct DocMapping {
    pub field_mappings: Vec<FieldMapping>,
    pub timestamp_field: String,
}

#[derive(serde::Deserialize, Debug, Clone, PartialEq)]
pub struct FieldMapping {
    pub name: String,
    pub r#type: String,
}

#[derive(serde::Deserialize, Debug, Clone, PartialEq)]
pub struct Source {
    pub version: String,
    pub source_id: String,
    pub num_pipelines: u32,
    pub enabled: bool,
    pub source_type: String,
    pub input_format: String,
}

#[derive(serde::Deserialize, Debug, Clone, PartialEq)]
pub struct IndexDescription {
    pub num_published_docs: u64,
    pub size_published_docs_uncompressed: u64,
    pub num_published_splits: u32,
    pub size_published_splits: u64,
}

#[derive(serde::Deserialize, Debug, Clone, PartialEq)]
pub enum SplitState {
    Staged,
    Published,
    MarkedForDeletion,
}

#[derive(serde::Deserialize, Debug, Clone, PartialEq)]
pub struct Split {
    pub split_state: SplitState,
}

#[derive(serde::Deserialize, Debug, Clone, PartialEq)]
pub struct ListSplitsResponse {
    pub splits: Vec<Split>,
    pub total_count: Option<u64>,
}
