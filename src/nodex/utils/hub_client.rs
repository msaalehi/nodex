use crate::network_config;
use crate::nodex::errors::NodeXError;
use hmac::{Hmac, Mac};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Url,
};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub struct HubClientConfig {
    pub base_url: String,
}

#[derive(Clone, Debug)]
pub struct HubClient {
    pub base_url: Url,
    pub instance: reqwest::Client,
}

impl HubClient {
    pub fn new(_config: &HubClientConfig) -> Result<Self, NodeXError> {
        let url = match Url::parse(&_config.base_url.to_string()) {
            Ok(v) => v,
            Err(e) => {
                log::error!("{:?}", e);
                return Err(NodeXError {});
            }
        };
        let client: reqwest::Client = reqwest::Client::new();

        Ok(HubClient {
            instance: client,
            base_url: url,
        })
    }

    fn auth_headers(&self, payload: String) -> Result<HeaderMap, NodeXError> {
        let config = network_config();
        let secret = config.inner.lock().unwrap().get_secretk_key().unwrap();
        let mut mac = match HmacSha256::new_from_slice(secret.as_bytes()) {
            Ok(v) => v,
            Err(_) => {
                return Err(NodeXError {});
            }
        };
        mac.update(payload.as_bytes());
        let signature = &hex::encode(mac.finalize().into_bytes());
        let mut headers = HeaderMap::new();
        headers.insert(
            "X-Nodex-Signature",
            HeaderValue::from_str(signature).unwrap(),
        );
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        Ok(headers)
    }

    #[allow(dead_code)]
    pub async fn get(&self, _path: &str) -> Result<reqwest::Response, NodeXError> {
        let url = self.base_url.join(_path);
        let headers = self.auth_headers("".to_string());
        if let Err(e) = headers {
            log::error!("{:?}", e);
            return Err(NodeXError {});
        }
        match self
            .instance
            .get(&url.unwrap().to_string())
            .headers(headers.unwrap())
            .send()
            .await
        {
            Ok(v) => Ok(v),
            Err(e) => {
                log::error!("{:?}", e);
                Err(NodeXError {})
            }
        }
    }

    pub async fn post(&self, _path: &str, body: &str) -> Result<reqwest::Response, NodeXError> {
        let url = self.base_url.join(_path);
        let headers = self.auth_headers(body.to_string());
        if let Err(e) = headers {
            log::error!("{:?}", e);
            return Err(NodeXError {});
        }
        match self
            .instance
            .post(&url.unwrap().to_string())
            .headers(headers.unwrap())
            .body(body.to_string())
            .send()
            .await
        {
            Ok(v) => Ok(v),
            Err(e) => {
                log::error!("{:?}", e);
                Err(NodeXError {})
            }
        }
    }

    #[allow(dead_code)]
    pub async fn put(&self, _path: &str) -> Result<reqwest::Response, NodeXError> {
        let url = self.base_url.join(_path);
        let headers = self.auth_headers("".to_string());
        if let Err(e) = headers {
            log::error!("{:?}", e);
            return Err(NodeXError {});
        }
        match self
            .instance
            .put(&url.unwrap().to_string())
            .headers(headers.unwrap())
            .send()
            .await
        {
            Ok(v) => Ok(v),
            Err(e) => {
                log::error!("{:?}", e);
                Err(NodeXError {})
            }
        }
    }

    #[allow(dead_code)]
    pub async fn delete(&self, _path: &str) -> Result<reqwest::Response, NodeXError> {
        let url = self.base_url.join(_path);
        let headers = self.auth_headers("".to_string());
        if let Err(e) = headers {
            log::error!("{:?}", e);
            return Err(NodeXError {});
        }
        match self
            .instance
            .delete(&url.unwrap().to_string())
            .headers(headers.unwrap())
            .send()
            .await
        {
            Ok(v) => Ok(v),
            Err(e) => {
                log::error!("{:?}", e);
                Err(NodeXError {})
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct Res {
        origin: String,
    }

    #[actix_rt::test]
    #[ignore]
    async fn it_should_success_get() {
        let client_config: HubClientConfig = HubClientConfig {
            base_url: "https://httpbin.org".to_string(),
        };

        let client = match HubClient::new(&client_config) {
            Ok(v) => v,
            Err(_) => panic!(),
        };

        let res = match client.get("/get").await {
            Ok(v) => v,
            Err(_) => panic!(),
        };

        let json: Res = match res.json().await {
            Ok(v) => v,
            Err(_) => panic!(),
        };

        assert!(!json.origin.is_empty());
    }

    #[actix_rt::test]
    #[ignore]
    async fn it_should_success_post() {
        let client_config: HubClientConfig = HubClientConfig {
            base_url: "https://httpbin.org".to_string(),
        };

        let client = match HubClient::new(&client_config) {
            Ok(v) => v,
            Err(_) => panic!(),
        };

        let res = match client.post("/post", r#"{"key":"value"}"#).await {
            Ok(v) => v,
            Err(_) => panic!(),
        };

        let json: Res = match res.json().await {
            Ok(v) => v,
            Err(_) => panic!(),
        };

        assert!(!json.origin.is_empty());
    }

    #[actix_rt::test]
    #[ignore]
    async fn it_should_success_put() {
        let client_config: HubClientConfig = HubClientConfig {
            base_url: "https://httpbin.org".to_string(),
        };

        let client = match HubClient::new(&client_config) {
            Ok(v) => v,
            Err(_) => panic!(),
        };

        let res = match client.put("/put").await {
            Ok(v) => v,
            Err(_) => panic!(),
        };

        let json: Res = match res.json().await {
            Ok(v) => v,
            Err(_) => panic!(),
        };

        assert!(!json.origin.is_empty());
    }

    #[actix_rt::test]
    #[ignore]
    async fn it_should_success_delete() {
        let client_config: HubClientConfig = HubClientConfig {
            base_url: "https://httpbin.org".to_string(),
        };

        let client = match HubClient::new(&client_config) {
            Ok(v) => v,
            Err(_) => panic!(),
        };

        let res = match client.delete("/delete").await {
            Ok(v) => v,
            Err(_) => panic!(),
        };

        let json: Res = match res.json().await {
            Ok(v) => v,
            Err(_) => panic!(),
        };

        assert!(!json.origin.is_empty());
    }
}
