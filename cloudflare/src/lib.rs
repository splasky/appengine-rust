// Copyright 2026 HY Chang
// SPDX-License-Identifier: Apache-2.0

use worker::*;

#[event(fetch)]
async fn fetch(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    let url = req.url()?;
    let path = url.path().to_string();

    match path.as_str() {
        "/" => {
            let info = serde_json::json!({
                "service": "appengine-rust",
                "version": env!("CARGO_PKG_VERSION"),
            });
            Response::from_json(&info)
        }
        "/health" => Response::ok("healthy"),
        _ => Response::error("Not Found", 404),
    }
}
