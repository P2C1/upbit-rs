mod upbit;
use std::{collections::HashMap, env};

use crate::upbit::Client;

#[tokio::main]
async fn main() {
    dotenv::vars();

    ///
    /// step 1
    ///
    let secret_key = env::var("SECRET_KEY").expect("enviroment variable SECRET_KEY is not set!");
    let access_key = env::var("ACCESS_KEY").expect("enviroment variable ACCESS_KEY is not set!");
    println!("secret key : {} \naccess key{}", secret_key, access_key);

    ///
    /// step 2
    ///
    let client = Client::new("hello", "world");
    println!("non-param jwt >>>> \n{}", client.generate_jwt(None));

    ///
    /// step 3
    ///
    let mut hashmap = HashMap::<&str, &str>::new();
    hashmap.insert("hello", "world");
    hashmap.insert("ya", "ho");
    println!("param jwt >>>> \n{}", client.generate_jwt(Some(&hashmap)));
}
