use std::env;
use dotenvy::dotenv;

use crate::models::messages::Message;

fn token() -> String {
    dotenv().ok();
    env::var("TEST_TOKEN").unwrap()
}


