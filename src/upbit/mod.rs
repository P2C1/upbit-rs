use std::collections::HashMap;

use uuid::Uuid;




///
/// Payload Stubs..
/// 
pub trait Payload{
    fn to_jwt(&self) -> String;
}

#[derive(Debug)]
pub struct NonParamPayload{
    access_key: String,
    nounce: String,
}


impl Payload for NonParamPayload{
    fn to_jwt(&self) -> String{
        "".to_string()
    }
}

#[derive(Debug)]
pub struct ParamPayload{
    access_key: String,
    nounce: String,
    query_hash_alg: String,
    query_hash: String,
}

impl Payload for ParamPayload{
    fn to_jwt(&self) -> String{
        "".to_string()
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
    pub fn new(access_key: String, secret_key: String) -> Self{
        Client{
            access_key: access_key.to_string(),
            secret_key: secret_key.to_string(),
            nounce: Uuid::nil().to_string(),
            non_param_payload: NonParamPayload { access_key: access_key.to_string(), nounce: Uuid::nil().to_string() }
        }
    }

    fn generate_jwt(&self, query: Option<HashMap<String, String>>) -> String
    {
        match query{
            None => NonParamPayload { 
                access_key : self.access_key.clone(),
                 nounce: self.nounce.clone() 
            }.to_jwt(),
            Some(map) => ParamPayload{
                access_key: self.access_key.clone(),
                nounce: self.nounce.clone(),
                query_hash_alg: "SHA512".to_string(),
                query_hash: "TBD".to_string(),
            }.to_jwt(),
        }
    }
}    

