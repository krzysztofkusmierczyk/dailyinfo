#![allow(dead_code)]
use dotenvy::dotenv;
use providers::greeter::greet;
use slack::{
    Block, HeaderBlock, Message, SectionBlock, SlackWebhookClient, SlackWebhookUrl, TextObject,
};
use std::env;
mod providers;
mod slack;

fn main() {
    dotenv().ok();

    let webhook_url = env::var("SLACK_WEBHOOK").expect("SLACK_WEBHOOK not set");

    let client = SlackWebhookClient::new(SlackWebhookUrl::new(webhook_url));
    let mut message = Message::default();
    message
        .with_block(Block::Header(HeaderBlock::with_text(TextObject::plain(
            greet(),
        ))))
        .with_block(Block::Divider)
        .with_block(Block::Section(SectionBlock::with_text(
            TextObject::markdown("Markdown text\nand another!".to_owned()),
        )))
        .with_block(Block::Divider)
        .with_block(Block::Section(SectionBlock::with_fields(vec![
            TextObject::markdown("*Title:*\nText".to_owned()),
            TextObject::markdown("*Title 2:*\nText 2".to_owned()),
        ])));
    client.send(&message).expect("Could not send slack message");
}
