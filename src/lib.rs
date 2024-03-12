pub mod d1;
pub mod repository;

use anyhow::{bail, Error as AnyhowError, Result as AnyhowResult};
use d1::{BulkInsertParams, D1};
use garde::Validate;
use repository::{CreateParams, Options, Repository, UpdateParams};
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use worker::*;

const BINDING_NAME: &str = "DB";

#[derive(Validate, Deserialize, Debug)]
struct CreateRequest {
    #[garde(required)]
    pub post_id: Option<i32>,

    #[garde(required)]
    pub short_text: Option<String>,

    #[garde(required)]
    pub sample_id: Option<i32>,
}

impl TryFrom<CreateRequest> for CreateParams {
    type Error = AnyhowError;
    fn try_from(req: CreateRequest) -> AnyhowResult<Self> {
        let post_id = match req.post_id {
            Some(id) => id,
            None => bail!("post_id is required"),
        };

        let short_text = match req.short_text {
            Some(text) => text,
            None => bail!("short_text is required"),
        };

        let sample_id = match req.sample_id {
            Some(id) => id,
            None => bail!("sample_id is required"),
        };

        Ok(CreateParams::new(post_id, short_text, sample_id))
    }
}

#[derive(Deserialize, Debug)]
struct UpdateRequest {
    pub post_id: Option<i32>,
    pub short_text: Option<String>,
    pub sample_id: Option<i32>,
}
impl UpdateParams {
    fn try_new(id: Option<&String>, update_request: UpdateRequest) -> AnyhowResult<Self> {
        let id_str = match id {
            Some(id) => id,
            None => bail!("id is required"),
        };

        let id = match id_str.parse::<i32>() {
            Ok(id) => id,
            Err(e) => bail!("failed to parse id: {}", e),
        };

        Ok(Self {
            id,
            post_id: update_request.post_id,
            short_text: update_request.short_text,
            sample_id: update_request.sample_id,
        })
    }
}

#[derive(Deserialize, Debug, Validate)]
pub struct BulkInsertRequest {
    #[garde(required)]
    pub row_count: Option<i32>,
}
impl TryFrom<BulkInsertRequest> for BulkInsertParams {
    type Error = AnyhowError;
    fn try_from(req: BulkInsertRequest) -> AnyhowResult<Self> {
        let row_count = match req.row_count {
            Some(count) => count,
            None => bail!("row_count is required"),
        };

        Ok(BulkInsertParams::new(row_count))
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct GenericResponse {
    status: u16,
    message: String,
}

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    Router::new()
        .get_async("/get", handle_get)
        .post_async("/post", handle_post)
        .post_async("/bulk_insert", handle_bulk_insert)
        .put_async("/update/:id", handle_put)
        .delete_async("/delete/:id", handle_delete)
        .delete_async("/delete", handle_all_delete)
        .run(req, env)
        .await
}

pub async fn handle_get(_: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    let d1 = D1::from(ctx.env.d1(BINDING_NAME)?);
    let options = Options::new(100);

    let result = match d1.get(options).await {
        Ok(result) => serde_wasm_bindgen::to_value(&result),
        Err(e) => {
            return Response::from_json(&GenericResponse {
                status: 400,
                message: e.to_string(),
            });
        }
    };

    Response::from_json(&GenericResponse {
        status: 200,
        message: format!("You reached a GET route! {:?}", result),
    })
}

pub async fn handle_post(mut request: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    let data = from_str::<CreateRequest>(request.text().await?.as_str());

    let create_request = match data {
        Ok(req) => req,
        Err(e) => {
            return Response::from_json(&GenericResponse {
                status: 400,
                message: e.to_string(),
            });
        }
    };

    match create_request.validate(&()) {
        Ok(_) => {}
        Err(e) => {
            return Response::from_json(&GenericResponse {
                status: 400,
                message: e.to_string(),
            });
        }
    }

    let create_params = match CreateParams::try_from(create_request) {
        Ok(params) => params,
        Err(e) => {
            return Response::from_json(&GenericResponse {
                status: 400,
                message: e.to_string(),
            });
        }
    };

    let d1 = D1::from(ctx.env.d1(BINDING_NAME)?);
    let result = match d1.create(create_params).await {
        Ok(result) => serde_wasm_bindgen::to_value(&result).unwrap(),
        Err(e) => {
            return Response::from_json(&GenericResponse {
                status: 500,
                message: e.to_string(),
            });
        }
    };
    Response::from_json(&GenericResponse {
        status: 200,
        message: format!("You reached a POST route! {:?}", result),
    })
}

pub async fn handle_put(mut request: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    let id = ctx.param("id");
    let data = from_str::<UpdateRequest>(request.text().await?.as_str());

    let update_request = match data {
        Ok(req) => req,
        Err(e) => {
            return Response::from_json(&GenericResponse {
                status: 400,
                message: e.to_string(),
            });
        }
    };
    let update_params = UpdateParams::try_new(id, update_request);
    let update_params = match update_params {
        Ok(params) => Some(params),
        Err(e) => {
            return Response::from_json(&GenericResponse {
                status: 400,
                message: e.to_string(),
            });
        }
    };

    let d1 = D1::from(ctx.env.d1(BINDING_NAME)?);
    let result = match d1.update(update_params.unwrap()).await {
        Ok(result) => serde_wasm_bindgen::to_value(&result).unwrap(),
        Err(e) => {
            return Response::from_json(&GenericResponse {
                status: 500,
                message: e.to_string(),
            });
        }
    };

    Response::from_json(&GenericResponse {
        status: 200,
        message: format!("You reached a PUT route! {:?}", result),
    })
}

pub async fn handle_delete(_: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    let param = ctx.param("id");

    let id = match param.unwrap().parse::<i32>() {
        Ok(id) => id,
        Err(e) => {
            return Response::from_json(&GenericResponse {
                status: 400,
                message: e.to_string(),
            });
        }
    };

    let d1 = D1::from(ctx.env.d1(BINDING_NAME)?);
    let result = match d1.delete(id).await {
        Ok(result) => serde_wasm_bindgen::to_value(&result).unwrap(),
        Err(e) => {
            return Response::from_json(&GenericResponse {
                status: 500,
                message: e.to_string(),
            });
        }
    };

    Response::from_json(&GenericResponse {
        status: 200,
        message: format!("You reached a DELETE route! {:?}", result),
    })
}

pub async fn handle_bulk_insert(
    mut request: Request,
    ctx: RouteContext<()>,
) -> worker::Result<Response> {
    let data = from_str::<BulkInsertRequest>(request.text().await?.as_str());
    let bulk_insert_request = match data {
        Ok(req) => req,
        Err(e) => {
            return Response::from_json(&GenericResponse {
                status: 400,
                message: e.to_string(),
            });
        }
    };

    match bulk_insert_request.validate(&()) {
        Ok(_) => {}
        Err(e) => {
            return Response::from_json(&GenericResponse {
                status: 400,
                message: e.to_string(),
            });
        }
    }

    let bulk_insert_params = match BulkInsertParams::try_from(bulk_insert_request) {
        Ok(params) => params,
        Err(e) => {
            return Response::from_json(&GenericResponse {
                status: 400,
                message: e.to_string(),
            });
        }
    };

    let d1 = D1::from(ctx.env.d1(BINDING_NAME)?);
    let result = match d1.bulk_insert(bulk_insert_params).await {
        Ok(result) => serde_wasm_bindgen::to_value(&result).unwrap(),
        Err(e) => {
            return Response::from_json(&GenericResponse {
                status: 500,
                message: e.to_string(),
            });
        }
    };

    Response::from_json(&GenericResponse {
        status: 200,
        message: format!("You reached a BULK INSERT route! {:?}", result),
    })
}

pub async fn handle_all_delete(_: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    let d1 = D1::from(ctx.env.d1(BINDING_NAME)?);
    let result = match d1.all_delete().await {
        Ok(result) => serde_wasm_bindgen::to_value(&result).unwrap(),
        Err(e) => {
            return Response::from_json(&GenericResponse {
                status: 500,
                message: e.to_string(),
            });
        }
    };

    Response::from_json(&GenericResponse {
        status: 200,
        message: format!("You reached a DELETE route! {:?}", result),
    })
}
