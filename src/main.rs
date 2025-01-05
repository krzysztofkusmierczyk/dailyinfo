#![allow(dead_code)]
use dotenvy::dotenv;
mod logic;
mod providers;
mod slack;

mod cli;
use std::env;
fn main() {
    dotenv().ok();

    let webhook_url = env::var("SLACK_WEBHOOK").expect("SLACK_WEBHOOK not set");

    let cli_args = cli::Cli::parsed();
    logic::send_message(&webhook_url, cli_args.calendar_date);
}
