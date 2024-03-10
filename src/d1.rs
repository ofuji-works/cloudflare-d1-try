use anyhow::{anyhow, bail, Result};
use async_trait::async_trait;
use serde::Deserialize;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::JsValue;
use worker::*;

use crate::repository::*;

pub struct D1 {
    db: D1Database,
}

impl From<D1Database> for D1 {
    fn from(db: D1Database) -> Self {
        D1 { db }
    }
}

impl CreateParams {
    fn js_values(&self) -> Result<Vec<JsValue>> {
        let name = to_value(&self.name).or(Err(anyhow!("failed set name parameter")))?;

        Ok(vec![name])
    }
}

impl UpdateParams {
    fn js_values(&self) -> Result<Vec<JsValue>> {
        let id = to_value(&self.id).or(Err(anyhow!("failed set id parameter")))?;
        let name = to_value(&self.name).or(Err(anyhow!("failed set name parameter")))?;

        Ok(vec![id, name])
    }
}

#[async_trait(?Send)]
impl Repository for D1 {
    async fn get<T>(&self, options: Options) -> Result<Vec<Vec<T>>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let statement = self.db.prepare("SELECT * FROM d1 LIMIT ?");
        let limit = to_value(&options.limit()).or(Err(anyhow!("failed set limit parameter")))?;
        let query = statement
            .bind(&[limit])
            .or(Err(anyhow!("failed generate query")))?;
        let result = query.raw().await;

        return match result {
            Ok(result) => Ok(result),
            Err(e) => bail!("Error: {}", e),
        };
    }
    async fn create(&self, params: CreateParams) -> Result<QueryResult> {
        let statement = self.db.prepare("INSERT INTO d1 (name) VALUES (?)");
        let query = statement
            .bind(&params.js_values()?)
            .or(Err(anyhow!("failed generate query")))?;
        let result = query.run().await.or(Err(anyhow!("failed run query")))?;

        Ok(QueryResult::from(result))
    }
    async fn update(&self, params: UpdateParams) -> Result<QueryResult> {
        let statement = self.db.prepare("UPDATE d1 SET name = ? WHERE id = ?");
        let query = statement
            .bind(&params.js_values()?)
            .or(Err(anyhow!("failed generate query")))?;
        let result = query.run().await.or(Err(anyhow!("failed run query")))?;

        Ok(QueryResult::from(result))
    }
    async fn delete(&self, id: i32) -> Result<QueryResult> {
        let statement = self.db.prepare("DELETE FROM d1 WHERE id = ?");
        let query = statement
            .bind(&[to_value(&id).or(Err(anyhow!("failed set id parameter")))?])
            .or(Err(anyhow!("failed generate query")))?;
        let result = query.run().await.or(Err(anyhow!("failed run query")))?;

        Ok(QueryResult::from(result))
    }
}
