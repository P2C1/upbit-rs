mod upbit;
use std::env;

use crate::upbit::Client;

#[tokio::main]
async fn main(){
    dotenv::vars();

    let secret_key = env::var("SECRET_KEY").expect("enviroment variable SECRET_KEY is not set!");
    let access_key = env::var("ACCESS_KEY").expect("enviroment variable ACCESS_KEY is not set!");
    let cli = Client::new("hello", "world");
    println!("{}", secret_key);
    println!("{}", access_key);
}