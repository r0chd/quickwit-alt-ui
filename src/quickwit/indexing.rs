#[derive(serde::Deserialize, Debug, Clone, PartialEq)]
pub struct Indexing {
    pub num_staged_splits: Option<i64>,
}
