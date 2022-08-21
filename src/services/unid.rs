use crate::{unid::{errors::UNiDError, keyring, sidetree::payload::{OperationPayload, DIDCreateRequest, CommitmentKeys, DIDCreateResponse, DIDResolutionResponse}, utils::http_client::{HttpClient, HttpClientConfig}}, config::{AppConfig, SignKeyPair, UpdateKeyPair, RecoverKeyPair, EncryptKeyPair}};

pub struct UNiD {
    http_client: HttpClient
}

impl UNiD {
    pub fn new() -> Self {
        let client_config: HttpClientConfig = HttpClientConfig {
            base_url: "https://did.getunid.io".to_string(),
        };

        let client = match HttpClient::new(&client_config) {
            Ok(v) => v,
            Err(_) => panic!()
        };

        UNiD { http_client: client }
    }

    // NOTE: DONE
    pub async fn create_identifier(&self) -> Result<DIDCreateResponse, UNiDError> {
        let mut keyring = match keyring::mnemonic::MnemonicKeyring::create_keyring() {
            Ok(v) => v,
            Err(_) => return Err(UNiDError{}),
        };

        // NOTE: create payload
        let public = match keyring.get_sign_key_pair().to_public_key("signingKey", &vec!["auth", "general"]) {
            Ok(v) => v,
            Err(_) => return Err(UNiDError{}),
        };
        let update = match keyring.get_recovery_key_pair().to_jwk(false) {
            Ok(v) => v,
            Err(_) => return Err(UNiDError{}),
        };
        let recovery = match keyring.get_update_key_pair().to_jwk(false) {
            Ok(v) => v,
            Err(_) => return Err(UNiDError{}),
        };

        let payload = match OperationPayload::did_create_payload(&DIDCreateRequest {
            public_keys: vec![ public ],
            commitment_keys: CommitmentKeys {
                recovery,
                update,
            },
            service_endpoints: vec![],
        }) {
            Ok(v) => v,
            Err(_) => return Err(UNiDError{}),
        };

        let res = match self.http_client.post(&("/api/v1/operations"), &payload).await {
            Ok(v) => v,
            Err(_) => return Err(UNiDError{}),
        };

        let json = match res.json::<DIDCreateResponse>().await {
            Ok(v) => v,
            Err(_) => return Err(UNiDError{}),
        };

        // NOTE: save context
        keyring.save(&json.did_document.id);

        Ok(json)
    }

    // NOTE: DONE
    pub async fn find_identifier(&self, did: &str) -> Result<DIDResolutionResponse, UNiDError> {
        let res = match self.http_client.get(&(format!("/api/v1/identifiers/{}", &did))).await {
            Ok(v) => v,
            Err(_) => return Err(UNiDError{})
        };

        match res.json::<DIDResolutionResponse>().await {
            Ok(v) => Ok(v),
            Err(_) => Err(UNiDError{})
        }
    }

    pub  fn transfer(&self) -> Result<String, UNiDError> {
        Ok("NotImplemented".to_string())
    }
}