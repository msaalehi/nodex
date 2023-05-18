use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::services::internal::didcomm_signed::DIDCommSignedService;

// NOTE: POST /internal/didcomm/signed-messages/verify
#[derive(Deserialize, Serialize)]
pub struct MessageContainer {
    message: Value,
}

pub async fn handler(
    _req: HttpRequest,
    web::Json(json): web::Json<MessageContainer>,
) -> actix_web::Result<HttpResponse> {
    match DIDCommSignedService::verify(&json.message).await {
        Ok(v) => Ok(HttpResponse::Ok().json(&v)),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}
