use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

const DEFAULT_LIMIT: i32 = 100;

pub struct Options {
    pub limit: Option<i32>,
}
impl Options {
    pub fn new(limit: i32) -> Self {
        Self { limit: Some(limit) }
    }

    pub fn limit(&self) -> i32 {
        if let Some(limit) = self.limit {
            return limit;
        }

        DEFAULT_LIMIT
    }
}

#[derive(Serialize)]
pub struct QueryResult {
    result: String,
}
impl From<String> for QueryResult {
    fn from(result: String) -> Self {
        Self { result }
    }
}
impl QueryResult {
    pub fn result(&self) -> &str {
        &self.result
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TestData {
    id: i32,
    post_id: i32,
    short_text: String,
    created_at: String,
    updated_at: String,
    sample_id: i32,
}

pub struct CreateParams {
    pub post_id: i32,
    pub short_text: String,
    pub sample_id: i32,
}
impl CreateParams {
    pub fn new(post_id: i32, short_text: String, sample_id: i32) -> Self {
        Self {
            post_id,
            short_text,
            sample_id,
        }
    }
}

pub struct UpdateParams {
    pub id: i32,
    pub post_id: Option<i32>,
    pub short_text: Option<String>,
    pub sample_id: Option<i32>,
}
impl UpdateParams {
    pub fn new(
        id: i32,
        post_id: Option<i32>,
        short_text: Option<String>,
        sample_id: Option<i32>,
    ) -> Self {
        Self {
            id,
            post_id,
            short_text,
            sample_id,
        }
    }
}

#[async_trait(?Send)]
pub trait Repository {
    async fn get(&self, options: Options) -> Result<Vec<TestData>>;
    async fn create(&self, params: CreateParams) -> Result<QueryResult>;
    async fn update(&self, params: UpdateParams) -> Result<QueryResult>;
    async fn delete(&self, id: i32) -> Result<QueryResult>;
}
