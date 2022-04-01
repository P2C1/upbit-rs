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
    println!("res :: {:?}", client.get_account().await);

    // step 5 : test query_market_all
    println!("=================test query_market_all()================");
    println!("res :: {:?}", client.get_market_all(true).await);

    // step 6 : test query_order_chance
    println!("=================test query_order_chance()================");
    println!("res :: {:?}", client.get_orders_chance("KRW-BTC").await);

    // step 7 : test orders
    // side -> bid (매수), ask(매도)
    // 주문타입 -> limit (지정가 주문), price (시장가 매수), market : (시장가 매도)
    println!("==================test orders=============================");
    println!(
        "res :: {:?}",
        client
            .post_orders("KRW-BTC", "bid", "1", "6000", "limit", None)
            .await // 비트코인 1 개를 6000원에 매수
    );

    // step 7 : delete order
    // uuid 혹은 identifier 중 하나는 반드시 포함되어야 합니다
    println!("===================test delete order=======================");
    println!(
        "res :: {:?}",
        client
            .delete_order(Some("e2c04420-db90-4aab-a53a-46b0f2a8bd8a"), None)
            .await
    );
}
