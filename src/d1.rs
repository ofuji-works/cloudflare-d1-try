use anyhow::{anyhow, bail, Result};
use async_trait::async_trait;
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
        let post_id = to_value(&self.post_id).or(Err(anyhow!("failed set post_id parameter")))?;
        let short_text =
            to_value(&self.short_text).or(Err(anyhow!("failed set short_text parameter")))?;
        let sample_id =
            to_value(&self.sample_id).or(Err(anyhow!("failed set sample_id parameter")))?;

        Ok(vec![post_id, short_text, sample_id])
    }
}

impl UpdateParams {
    fn js_values(&self) -> Result<Vec<JsValue>> {
        let mut values = vec![];
        if let Some(post_id) = self.post_id {
            values.push(to_value(&post_id).or(Err(anyhow!("failed set post_id parameter")))?);
        }
        if let Some(short_text) = &self.short_text {
            values.push(to_value(&short_text).or(Err(anyhow!("failed set short_text parameter")))?);
        }
        if let Some(sample_id) = self.sample_id {
            values.push(to_value(&sample_id).or(Err(anyhow!("failed set sample_id parameter")))?);
        }

        values.push(to_value(&self.id).or(Err(anyhow!("failed set id parameter")))?);

        Ok(values)
    }
}

#[async_trait(?Send)]
impl Repository for D1 {
    async fn get(&self, options: Options) -> Result<Vec<TestData>> {
        let statement = self.db.prepare("SELECT * FROM test_table LIMIT ?;");
        let limit = to_value(&options.limit()).or(Err(anyhow!("failed set limit parameter")))?;
        let query = statement
            .bind(&[limit])
            .or(Err(anyhow!("failed generate query")))?;

        let result = match query.all().await {
            Ok(result) => result,
            Err(e) => bail!("Error: {}", e),
        };

        return match result.results::<TestData>() {
            Ok(result) => Ok(result),
            Err(e) => bail!("Error: {}", e),
        };
    }
    async fn create(&self, params: CreateParams) -> Result<QueryResult> {
        let statement = self
            .db
            .prepare("INSERT INTO test_table (post_id, short_text, sample_id) VALUES (?, ?, ?);");
        let query = statement
            .bind(&params.js_values()?)
            .or(Err(anyhow!("failed generate query")))?;
        let _result = query.run().await.or(Err(anyhow!("failed run query")))?;

        Ok(QueryResult::from(String::from("success")))
    }
    async fn update(&self, params: UpdateParams) -> Result<QueryResult> {
        let mut set_values_text = String::new();

        if params.post_id.is_some() {
            set_values_text.push_str("post_id = ?");
        }

        if params.short_text.is_some() {
            set_values_text.push_str(", short_text = ?");
        }

        if params.sample_id.is_some() {
            set_values_text.push_str(", sample_id = ?");
        }

        let statement = self
            .db
            .prepare(format!("UPDATE test_table SET {} WHERE id = ?;", set_values_text).as_str());
        let query = statement
            .bind(&params.js_values()?)
            .or(Err(anyhow!("failed generate query")))?;
        let _result = query.run().await.or(Err(anyhow!("failed run query")))?;

        Ok(QueryResult::from(String::from("success")))
    }
    async fn delete(&self, id: i32) -> Result<QueryResult> {
        let statement = self.db.prepare("DELETE FROM test_table WHERE id = ?");
        let query = statement
            .bind(&[to_value(&id).or(Err(anyhow!("failed set id parameter")))?])
            .or(Err(anyhow!("failed generate query")))?;
        let _result = query.run().await.or(Err(anyhow!("failed run query")))?;

        Ok(QueryResult::from(String::from("success")))
    }
}
