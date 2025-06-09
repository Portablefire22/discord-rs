use std::env;
use dotenvy::dotenv;

fn token() -> String {
    dotenv().ok();
    env::var("TEST_TOKEN").unwrap()
}

#[tokio::test]
async fn authorised_guild_get_channels() {
    dotenv().ok(); 
    let client = super::Client::new(token(), None).await;
    match client.get_guild_channels(String::from("1225605857413173309")).await {
        Ok(res) => {
            //println!("{:?}", res.get(0));
            ()
        },
        Err(err) => panic!("{:?}", err),
    }
}
