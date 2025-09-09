pub mod cluster;
pub mod index;
pub mod indexing;
pub mod query;

use crate::quickwit::{
    index::{IndexDescription, ListSplitsResponse, Split},
    query::QueryResponse,
};
use cluster::Cluster;
use index::Index;
use indexing::Indexing;
use reqwasm::http::Request;
use serde::de::DeserializeOwned;
use wasm_bindgen::JsValue;

pub struct QuickwitApi;

impl QuickwitApi {
    async fn get<T: DeserializeOwned>(path: &str) -> anyhow::Result<T> {
        Request::get(path)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!(e))?
            .json::<T>()
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    pub async fn cluster() -> anyhow::Result<Cluster> {
        Self::get("/api/v1/cluster")
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    pub async fn indexes() -> anyhow::Result<Vec<Index>> {
        Self::get("/api/v1/indexes")
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    pub async fn indexing() -> anyhow::Result<Indexing> {
        Self::get("/api/v1/indexing")
            .await
            .map_err(|e| anyhow::anyhow!(e))
    }

    pub async fn index_detail(index_id: &str) -> anyhow::Result<Index> {
        let path = format!("/api/v1/indexes/{index_id}");
        Self::get(&path).await.map_err(|e| anyhow::anyhow!(e))
    }

    pub async fn index_describe(index_id: &str) -> anyhow::Result<IndexDescription> {
        let path = format!("/api/v1/indexes/{}/describe", index_id);
        Self::get(&path).await.map_err(|e| anyhow::anyhow!(e))
    }

    pub async fn index_splits(index_id: &str) -> anyhow::Result<ListSplitsResponse> {
        let path = format!("/api/v1/indexes/{}/splits", index_id);
        Self::get(&path).await.map_err(|e| anyhow::anyhow!(e))
    }

    pub async fn query(
    index_id: &str,
    query_str: &str,
    max_hits: usize,
    sort_by_field: &str,
) -> anyhow::Result<QueryResponse> {
    let encoded_index = urlencoding::encode(index_id);
    let encoded_query = urlencoding::encode(query_str);
    let encoded_sort = urlencoding::encode(sort_by_field);

    let mut query_params = format!("query={}&max_hits={}", encoded_query, max_hits);
    if !sort_by_field.is_empty() {
        query_params.push_str(&format!("&sort_by_field={}", encoded_sort));
    }

    let path = format!(
        "/api/v1/{}/search?{}",
        encoded_index, query_params
    );

    if let Some(win) = web_sys::window() {
        if let Ok(history) = win.history() {
            let mut url = format!(
                "?index={}&query={}&max_hits={}",
                encoded_index, encoded_query, max_hits
            );
            if !sort_by_field.is_empty() {
                url.push_str(&format!("&sort_by_field={}", encoded_sort));
            }
            let _ = history.push_state_with_url(&JsValue::NULL, "", Some(&url));
        }
    }

    Self::get(&path).await.map_err(|e| anyhow::anyhow!(e))
}
}
