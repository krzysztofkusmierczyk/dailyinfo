use dotenvy::dotenv;
use slack::{Block, Message, SlackWebhookClient, SlackWebhookUrl, TextObject, TextType};
use std::env;
mod slack;

fn main() {
    dotenv().ok();

    let webhook_url = env::var("SLACK_WEBHOOK").expect("SLACK_WEBHOOK not set");

    let client = SlackWebhookClient::new(SlackWebhookUrl::new(webhook_url));
    let message = Message::new("Test".to_owned()).with_blocks(vec![Block::Header(
        TextObject::plain("test text :crab:".to_string()),
    )]);
    client.send(&message).expect("Could not send slack message");
}
