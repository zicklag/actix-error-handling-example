use actix_web as aw;
use serde_json as sj;
use sj::json;

#[derive(thiserror::Error, Debug)]
pub enum BpimError {
    #[error("{message}")]
    WithStatus {
        status: aw::http::StatusCode,
        message: String,
    },
}

pub fn handle_err(anyhow_err: anyhow::Error) -> aw::HttpResponse {
    if let Some(bpim_err) = anyhow_err.downcast_ref::<BpimError>() {
        match bpim_err {
            BpimError::WithStatus { status, message: _ } => aw::HttpResponse::build(status.clone())
                .json(json!({
                    "err": anyhow_err.to_string(),
                    "status": json_status(status),
                    "causedBy": anyhow_err.chain().skip(1).map(|e| e.to_string()).collect::<Vec<_>>(),
                })),
        }
    } else {
        aw::HttpResponse::InternalServerError().json(json!({
            "err": anyhow_err.to_string(),
            "status": json_status(&http::StatusCode::INTERNAL_SERVER_ERROR),
            "causedBy": anyhow_err.chain().skip(1).map(|e| e.to_string()).collect::<Vec<_>>(),
        }))
    }
}

fn json_status(status: &http::StatusCode) -> sj::Value {
    json!({
        "code": status.as_u16(),
        "text": status.to_string(),
    })
}
