mod upbit;
use std::{env, collections::HashMap};

use crate::upbit::Client;

#[tokio::main]
async fn main(){
    dotenv::vars();

    let secret_key = env::var("SECRET_KEY").expect("enviroment variable SECRET_KEY is not set!");
    let access_key = env::var("ACCESS_KEY").expect("enviroment variable ACCESS_KEY is not set!");
    let client = Client::new("hello", "world");
    println!("{}", client.generate_jwt(None));
    
    let mut hashmap = HashMap::<&str, &str>::new();
    hashmap.insert("hello", "world");
    hashmap.insert("yao", "ho");
    println!("{}", client.generate_jwt(Some(&hashmap)));

}