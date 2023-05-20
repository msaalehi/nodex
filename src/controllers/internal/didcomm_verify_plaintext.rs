use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::services::internal::didcomm_plaintext::DIDCommPlaintextService;

// NOTE: POST /internal/didcomm/plaintext-messages/verify
#[derive(Deserialize, Serialize)]
pub struct MessageContainer {
    message: Value,
}

pub async fn handler(
    _req: HttpRequest,
    web::Json(json): web::Json<MessageContainer>,
) -> actix_web::Result<HttpResponse> {
    match DIDCommPlaintextService::verify(&json.message) {
        Ok(v) => Ok(HttpResponse::Ok().json(&v)),
        Err(_) => Ok(HttpResponse::InternalServerError().finish()),
    }
}
