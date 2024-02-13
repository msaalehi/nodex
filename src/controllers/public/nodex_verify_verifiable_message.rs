use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::usecase::verifiable_message_usecase::VerifiableMessageUseCase;
use crate::{
    services::{
        internal::did_vc::DIDVCService, nodex::NodeX,
        project_verifier::ProjectVerifierImplOnNetworkConfig,
    },
    usecase::verifiable_message_usecase::VerifyVerifiableMessageUseCaseError,
};

// NOTE: POST /verify-verifiable-message
#[derive(Deserialize, Serialize)]
pub struct MessageContainer {
    message: String,
}

pub async fn handler(
    _req: HttpRequest,
    web::Json(json): web::Json<MessageContainer>,
) -> actix_web::Result<HttpResponse> {
    let usecase = VerifiableMessageUseCase::new(
        Box::new(ProjectVerifierImplOnNetworkConfig::new()),
        Box::new(NodeX::new()),
        DIDVCService::new(NodeX::new()),
    );

    match usecase.verify(&json.message).await {
        Ok(v) => Ok(HttpResponse::Ok().body(v)),
        Err(e) => match e {
            VerifyVerifiableMessageUseCaseError::VerificationFailed => {
                Ok(HttpResponse::Unauthorized().finish())
            }
            VerifyVerifiableMessageUseCaseError::Other(e) => {
                log::error!("{:?}", e);
                Ok(HttpResponse::InternalServerError().finish())
            }
        },
    }
}