use std::env;
use http::StatusCode;

use anyhow::{bail, Result, Context};
use convert_case::{Case, Casing};
use serde_json::json;
use isahc::{prelude::*, Request};
use serde::{Deserialize};

pub(crate) fn create_draft(title: String) -> Result<u128> {
    let devto_token = env::var("DEVTO_TOKEN")
        .with_context(|| "You should export `DEVTO_TOKEN` as env var")?; // TODO: use clap and required

    let data = json!({
        "article": {
            "title": title.to_case(Case::Title),
            "body_markdown": "",
            "published": false,
            "tags": []
        }
    }).to_string();

    let mut response = Request::post("https://dev.to/api/articles")
        .header("Content-Type", "application/json")
        .header("api-key", devto_token)
        .body(data.as_str())?
        .send()?;

    if response.status() != StatusCode::CREATED {
        bail!("Could not create an article: {}", response.text()?);
    }

    #[derive(Deserialize)]
    struct Response {
        id: u128,
    }
    Ok(response.json::<Response>()?.id)
}
