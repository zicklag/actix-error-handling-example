#![allow(dead_code)]

use actix_web as aw;
use anyhow::Context;
use serde_json as sj;

mod error;
use error::{handle_err, BpimError};

#[actix_rt::main]
pub async fn run() -> std::io::Result<()> {
    aw::HttpServer::new(|| {
        aw::App::new()
            .route("/{option}", aw::web::get().to(handle_create))
            .route("/", aw::web::get().to(handle_create))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}

async fn handle_create(req: aw::HttpRequest) -> impl aw::Responder {
    match create(req).await {
        Ok(value) => aw::HttpResponse::Ok().json(value),
        Err(anyhow_err) => handle_err(anyhow_err),
    }
}

async fn create(req: aw::HttpRequest) -> anyhow::Result<sj::Value> {
    match req.match_info().get("option").unwrap_or("work") {
        "add_hoc" => this_will_break_add_hoc_err(),
        "break_unexpectedly" => this_will_fail_unexpectedly(),
        "break_with_status" => this_will_break_with_status().context("Something failed"),
        _ => this_will_work(),
    }
}

fn this_will_break_add_hoc_err() -> anyhow::Result<sj::Value> {
    Err(anyhow::format_err!("Add-hoc error!")).context("Couldn't do something")
}

fn this_will_work() -> anyhow::Result<sj::Value> {
    Ok(sj::from_str(r#"{ "hello": "world"}"#)?)
}

fn this_will_break_with_status() -> anyhow::Result<sj::Value> {
    Err(BpimError::WithStatus {
        status: http::StatusCode::IM_A_TEAPOT,
        message: "Tea time!".into(),
    }
    .into())
}

fn this_will_fail_unexpectedly() -> anyhow::Result<sj::Value> {
    Ok(sj::from_str(r#"{ "hello": "world",}"#).context("Could not parse json")?)
}
