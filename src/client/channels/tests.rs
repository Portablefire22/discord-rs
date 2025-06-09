use std::env;
use dotenvy::dotenv;

fn token() -> String {
    dotenv().ok();
    env::var("TEST_TOKEN").unwrap()
}

#[tokio::test]
async fn authorised_channel_get_messages() {
    dotenv().ok(); 
    let client = super::Client::new(token(), None).await;
    match client.get_channel_messages(String::from("1225605857413173313"), None).await {
        Ok(res) => {
            //println!("{:?}", res.get(0));
            ()
        },
        Err(err) => panic!("{:?}", err),
    }
}

#[tokio::test]
async fn unauthorised_channel_get_messages() {
    dotenv().ok(); 
    let client = super::Client::new("owo".to_string(), None).await;
    match client.get_channel_messages(String::from("1225605857413173313"), None).await {
        Ok(res) => {
            //println!("{:?}", res.get(0));
            panic!("Retrieved messages without authorisation!")
        },
        Err(err) => (),
    }
}

#[tokio::test]
async fn authorised_get_channel() {
    dotenv().ok();
    let client = super::Client::new(token(), None).await;
    match client.get_channel("1225605857413173313".to_string()).await {
        Ok(res) => (),
        Err(err) => panic!("{:?}", err),
    }
}

#[tokio::test]
async fn unauthorised_get_channel() {
    dotenv().ok();
    let client = super::Client::new("owo".to_string(), None).await;
    match client.get_channel("1225605857413173313".to_string()).await {
        Ok(res) => panic!("Got channel without authorisation!"),
        Err(err) => (),
    }
}
