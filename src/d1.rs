use anyhow::{anyhow, bail, Result};
use async_trait::async_trait;
use serde::Deserialize;
use serde_wasm_bindgen::to_value;
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
    async fn create(&self) -> Result<()> {
        Ok(())
    }
    async fn update(&self) -> Result<()> {
        Ok(())
    }
    async fn delete(&self) -> Result<()> {
        Ok(())
    }
}
