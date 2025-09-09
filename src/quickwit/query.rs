#[derive(serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub struct QueryResponse {
    pub elapsed_time_micros: f64,
    pub hits: Vec<serde_json::Value>,
    pub num_hits: i64,
}
