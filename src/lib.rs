pub mod d1;
pub mod repository;

use d1::D1;
use repository::{Options, Repository};
use serde::{Deserialize, Serialize};
use worker::*;

const BINDING_NAME: &str = "DB";

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
        .put_async("/update", handle_put)
        .delete_async("/delete", handle_delete)
        .run(req, env)
        .await
}

pub async fn handle_get(_: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    let d1 = D1::from(ctx.env.d1(BINDING_NAME)?);
    let options = Options::new(100);

    let r = match d1.get(options).await {
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
        message: format!("You reached a GET route! {:?}", r),
    })
}

pub async fn handle_post(_: Request, _ctx: RouteContext<()>) -> worker::Result<Response> {
    Response::from_json(&GenericResponse {
        status: 200,
        message: "You reached a POST route!".to_string(),
    })
}

pub async fn handle_put(_: Request, _ctx: RouteContext<()>) -> worker::Result<Response> {
    Response::from_json(&GenericResponse {
        status: 200,
        message: "You reached a PUT route!".to_string(),
    })
}

pub async fn handle_delete(_: Request, _ctx: RouteContext<()>) -> worker::Result<Response> {
    Response::from_json(&GenericResponse {
        status: 200,
        message: "You reached a DELETE route!".to_string(),
    })
}
