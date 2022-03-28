use std::{collections::HashMap, sync::Arc};
use serde::Serialize;
use uuid::Uuid;
use jsonwebtoken::{Header, Algorithm, EncodingKey, encode};

pub trait Payload{
    fn to_jwt(&self, secret_key: &str) -> String;
}

#[derive(Debug, Serialize)]
pub struct NonParamPayload{
    access_key: String,
    nounce: String,
}


impl Payload for NonParamPayload{
    fn to_jwt(&self, secret_key: &str) -> String{
        let header = Header::new(Algorithm::HS512);
        encode(&header, &self, &EncodingKey::from_secret(secret_key.as_bytes())).unwrap()
    }
}

#[derive(Debug, Serialize)]
pub struct ParamPayload{
    access_key: String,
    nounce: String,
    query_hash_alg: String,
    query_hash: String,
}

impl Payload for ParamPayload{
    fn to_jwt(&self, secret_key: &str) -> String{
        let header = Header::new(Algorithm::HS512);
        let result = encode(&header, &self, &EncodingKey::from_secret(secret_key.as_bytes())).unwrap();
        result
    }
}

#[derive(Debug)]
pub struct Client{
    access_key: String,
    secret_key: String,
    nounce: String,
    non_param_payload: NonParamPayload
}

impl Client{
    pub fn new(access_key: &str, secret_key: &str) -> Self{
        Client{
            access_key: access_key.to_string(),
            secret_key: secret_key.to_string(),
            nounce: Uuid::nil().to_string(),
            non_param_payload: NonParamPayload { access_key: access_key.to_string(), nounce: Uuid::nil().to_string() }
        }
    }

    pub fn generate_jwt(&self, query: Option<&HashMap<&str, &str>>) -> String
    {
        match query{
            None => {
                    NonParamPayload { 
                    access_key : self.access_key.clone(),
                    nounce: self.nounce.clone() 
                }.to_jwt(&self.secret_key)
            },
            Some(qs_map) => {
                    let qs = qs_map.iter()
                                .map(|(k, v)| format!("{}={}", k, v))
                                .reduce(|a, b| format!("{}&{}", a, b))
                                .unwrap();
                    println!("{}", qs);
                    ParamPayload{
                    access_key: self.access_key.clone(),
                    nounce: self.nounce.clone(),
                    query_hash_alg: "SHA512".to_string(),
                    query_hash: "TBD".to_string(),
                }.to_jwt(&self.secret_key)
            },
        }
    }
}    

