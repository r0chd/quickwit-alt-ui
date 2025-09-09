pub mod cluster;
pub mod index;
pub mod indexing;
pub mod query;

use crate::quickwit::{
    index::{IndexDescription, ListSplitsResponse, Split},
    query::QueryResponse,
};
use cluster::Cluster;
use dioxus_logger::tracing::info;
use index::Index;
use indexing::Indexing;
use reqwasm::http::Request;
use serde::de::DeserializeOwned;
use wasm_bindgen::JsValue;

const BACKEND_URL: &str = env!("QW_BACKEND_URL");

pub struct QuickwitApi;

impl QuickwitApi {
    async fn get<T: DeserializeOwned>(path: &str) -> anyhow::Result<T> {
        let full_url = format!("{}{}", BACKEND_URL, path);
        Request::get(&full_url)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!(e))?
            .json::<T>()
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    pub async fn cluster() -> anyhow::Result<Cluster> {
        Self::get("/api/v1/cluster").await
    }

    pub async fn indexes() -> anyhow::Result<Vec<Index>> {
        Self::get("/api/v1/indexes").await
    }

    pub async fn indexing() -> anyhow::Result<Indexing> {
        Self::get("/api/v1/indexing").await
    }

    pub async fn index_detail(index_id: &str) -> anyhow::Result<Index> {
        let path = format!("/api/v1/indexes/{index_id}");
        Self::get(&path).await
    }

    pub async fn index_describe(index_id: &str) -> anyhow::Result<IndexDescription> {
        let path = format!("/api/v1/indexes/{}/describe", index_id);
        Self::get(&path).await
    }

    pub async fn index_splits(index_id: &str) -> anyhow::Result<ListSplitsResponse> {
        let path = format!("/api/v1/indexes/{}/splits", index_id);
        Self::get(&path).await
    }

    // New builder method
    pub fn query(index_id: &str) -> QueryBuilder {
        QueryBuilder::new(index_id)
    }
}

pub struct QueryBuilder<'a> {
    index_id: &'a str,
    query_str: String,
    max_hits: usize,
    sort_by_field: Option<String>,
    start_timestamp: Option<String>,
    end_timestamp: Option<String>,
}

impl<'a> QueryBuilder<'a> {
    pub fn new(index_id: &'a str) -> Self {
        Self {
            index_id,
            query_str: String::new(),
            max_hits: 20, // Default value
            sort_by_field: None,
            start_timestamp: None,
            end_timestamp: None,
        }
    }

    pub fn query(mut self, query: &str) -> Self {
        self.query_str = query.to_string();
        self
    }

    pub fn max_hits(mut self, max_hits: usize) -> Self {
        self.max_hits = max_hits;
        self
    }

    pub fn sort_by_field(mut self, field: &str) -> Self {
        self.sort_by_field = Some(field.to_string());
        self
    }

    pub fn time_range(mut self, start: &str, end: &str) -> Self {
        self.start_timestamp = Some(start.to_string());
        self.end_timestamp = Some(end.to_string());
        self
    }

    pub fn start_timestamp(mut self, start: &str) -> Self {
        self.start_timestamp = Some(start.to_string());
        self
    }

    pub fn end_timestamp(mut self, end: &str) -> Self {
        self.end_timestamp = Some(end.to_string());
        self
    }

    pub async fn execute(self) -> anyhow::Result<QueryResponse> {
        let encoded_index = urlencoding::encode(self.index_id);
        let encoded_query = urlencoding::encode(&self.query_str);

        let mut query_params = format!("query={}&max_hits={}", encoded_query, self.max_hits);

        if let Some(sort_field) = &self.sort_by_field {
            let encoded_sort = urlencoding::encode(sort_field);
            query_params.push_str(&format!("&sort_by_field={}", encoded_sort));
        }

        if let Some(start_ts) = &self.start_timestamp {
            query_params.push_str(&format!("&start_timestamp={}", start_ts));
        }

        if let Some(end_ts) = &self.end_timestamp {
            query_params.push_str(&format!("&end_timestamp={}", end_ts));
        }

        let path = format!("/api/v1/{}/search?{}", encoded_index, query_params);

        info!("{path}");

        if let Some(win) = web_sys::window() {
            if let Ok(history) = win.history() {
                let url = format!("?index={}&{}", encoded_index, query_params);
                let _ = history.push_state_with_url(&JsValue::NULL, "", Some(&url));
            }
        }

        QuickwitApi::get(&path).await
    }
}
