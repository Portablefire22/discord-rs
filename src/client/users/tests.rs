use std::env;
use dotenvy::dotenv;


fn token() -> String {
    dotenv().ok();
    env::var("TEST_TOKEN").unwrap()
}


