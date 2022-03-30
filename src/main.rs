mod upbit;
use crate::upbit::Client;
use std::{collections::HashMap, env, fmt::Result};

#[tokio::main]
async fn main() {
    dotenv::vars();

    // step 1 : display secret key, access key
    let secret_key = env::var("SECRET_KEY").expect("enviroment variable SECRET_KEY is not set!");
    let access_key = env::var("ACCESS_KEY").expect("enviroment variable ACCESS_KEY is not set!");
    println!("secret key : {} \naccess key{}", secret_key, access_key);

    // step 2 : test non param jwt
    let client = Client::new(&access_key, &secret_key);
    println!("non-param jwt >>>> \n{}", client.generate_jwt(None));

    // step 3 : test param jwt
    let mut hashmap = HashMap::from([("hello", "world".to_string()), ("ya", "ho".to_string())]);
    println!("param jwt >>>> \n{}", client.generate_jwt(Some(&hashmap)));

    // step 4 : test query_account
    println!("=================test query_account()=================");
    println!("res :: {:?}", client.query_account().await);

    // step 5 : test query_market_all
    println!("=================test query_market_all()================");
    println!("res :: {:?}", client.query_market_all(true).await);
}
