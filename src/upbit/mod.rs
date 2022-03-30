mod util;

use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use reqwest::{header::AUTHORIZATION, Client as reqwestClient};
use serde::Serialize;
use sha2::Sha512;
use std::collections::HashMap;
use std::str;
use urlencoding::encode as urlencode;
use uuid::Uuid;

pub trait Payload {
    fn to_jwt(&self, secret_key: &str) -> String;
}

#[derive(Debug, Serialize)]
pub struct NonParamPayload {
    access_key: String,
    nonce: String,
}

impl Payload for NonParamPayload {
    fn to_jwt(&self, secret_key: &str) -> String {
        let header = Header::new(Algorithm::HS512);
        encode(
            &header,
            &self,
            &EncodingKey::from_secret(secret_key.as_bytes()),
        )
        .unwrap()
    }
}

#[derive(Debug, Serialize)]
pub struct ParamPayload {
    access_key: String,
    nonce: String,
    query_hash_alg: String,
    query_hash: String,
}

impl Payload for ParamPayload {
    fn to_jwt(&self, secret_key: &str) -> String {
        let header = Header::new(Algorithm::HS512);
        let result = encode(
            &header,
            &self,
            &EncodingKey::from_secret(secret_key.as_bytes()),
        )
        .unwrap();
        result
    }
}

#[derive(Debug)]
pub struct Client {
    access_key: String,
    secret_key: String,
    nonce: String,
    non_param_payload: NonParamPayload,
    client: reqwestClient,
}

impl Client {
    const API_URL: &'static str = "https://api.upbit.com/v1";

    pub fn new(access_key: &str, secret_key: &str) -> Self {
        Client {
            access_key: access_key.to_string(),
            secret_key: secret_key.to_string(),
            nonce: Uuid::new_v4().to_string(),
            non_param_payload: NonParamPayload {
                access_key: access_key.to_string(),
                nonce: Uuid::nil().to_string(),
            },
            client: reqwest::Client::new(),
        }
    }

    pub fn generate_jwt(&self, query: Option<&HashMap<&str, String>>) -> String {
        match query {
            None => NonParamPayload {
                access_key: self.access_key.clone(),
                nonce: self.nonce.clone(),
            }
            .to_jwt(&self.secret_key),
            Some(qs_map) => {
                let qs = qs_map
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, v))
                    .reduce(|a, b| format!("{}&{}", a, b))
                    .unwrap();
                ParamPayload {
                    access_key: self.access_key.clone(),
                    nonce: self.nonce.clone(),
                    query_hash_alg: "SHA512".to_string(),
                    query_hash: util::hash::<Sha512>(&urlencode(&qs).as_bytes()),
                }
                .to_jwt(&self.secret_key)
            }
        }
    }

    pub async fn query_account(&self) -> serde_json::Value {
        let res = self
            .client
            .get(format!("{}/accounts", Client::API_URL))
            .header(AUTHORIZATION, format!("Bearer {}", self.generate_jwt(None)))
            .send()
            .await
            .unwrap();
        res.json::<serde_json::Value>().await.unwrap()
    }

    pub async fn query_market_all(&self, is_details: bool) -> serde_json::Value {
        let mut query = HashMap::from([("isDetails", is_details.to_string())]);
        let res = self
            .client
            .get(format!("{}/market/all", Client::API_URL))
            .header(
                AUTHORIZATION,
                format!("Bearer {}", self.generate_jwt(Some(&query))),
            )
            .send()
            .await
            .unwrap();
        res.json::<serde_json::Value>().await.unwrap()
    }
}
