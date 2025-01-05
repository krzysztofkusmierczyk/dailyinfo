#![allow(dead_code)]
use dotenvy::dotenv;
mod logic;
mod providers;
mod slack;

use std::env;
fn main() {
    dotenv().ok();

    let webhook_url = env::var("SLACK_WEBHOOK").expect("SLACK_WEBHOOK not set");
    logic::send_message(&webhook_url);
}
