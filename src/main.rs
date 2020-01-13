use actix_web as aw;
use serde_json as sj;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    aw::HttpServer::new(|| {
        aw::App::new()
            .route("/", aw::web::get().to(handle_create))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}

async fn handle_create(req: aw::HttpRequest) -> impl aw::Responder {
    match create(req).await {
        Ok(value) => aw::HttpResponse::Ok().json(value),
        Err(e) => aw::HttpResponse::Ok().json(sj::json!({"err": "There was an error!"})),
    }
}

async fn create(req: aw::HttpRequest) -> anyhow::Result<sj::Value> {

    let json = helper()?;

    Ok(json)
}

fn helper() -> anyhow::Result<sj::Value> {
    Ok(sj::from_str(r#"{ "hello": "world",}"#)?)
}