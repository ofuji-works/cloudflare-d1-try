use anyhow::Result;
use async_trait::async_trait;
use worker::D1Result;

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

pub struct QueryResult {
    result: D1Result,
}
impl From<D1Result> for QueryResult {
    fn from(result: D1Result) -> Self {
        Self { result }
    }
}
impl QueryResult {
    pub fn result(&self) -> &D1Result {
        &self.result
    }
}

pub struct CreateParams {
    pub name: String,
}
impl CreateParams {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

pub struct UpdateParams {
    pub id: i32,
    pub name: String,
}

#[async_trait(?Send)]
pub trait Repository {
    async fn get<T>(&self, options: Options) -> Result<Vec<Vec<T>>>
    where
        T: for<'de> serde::Deserialize<'de>;
    async fn create(&self, params: CreateParams) -> Result<QueryResult>;
    async fn update(&self, params: UpdateParams) -> Result<QueryResult>;
    async fn delete(&self, id: i32) -> Result<QueryResult>;
}
