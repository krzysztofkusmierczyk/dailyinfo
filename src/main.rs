#![allow(dead_code)]
use chrono::Utc;
use dotenvy::dotenv;
use providers::{calendar::get_calendar_day, greeter::greet};
use slack::{
    Block, HeaderBlock, Message, SectionBlock, SlackWebhookClient, SlackWebhookUrl, TextObject,
};
use std::env;
mod providers;
mod slack;

fn main() {
    dotenv().ok();

    let webhook_url = env::var("SLACK_WEBHOOK").expect("SLACK_WEBHOOK not set");

    let calendar_day =
        get_calendar_day(Utc::now().date_naive()).expect("Could not load calendar day information");

    let client = SlackWebhookClient::new(SlackWebhookUrl::new(webhook_url));
    let mut message = Message::default();
    message
        .with_block(Block::Header(HeaderBlock::with_text(TextObject::plain(
            greet(),
        ))))
        .with_block(Block::Divider)
        .with_block(Block::Section(SectionBlock::with_text(
            TextObject::markdown("*Dziś imieniny obchodzą:*"),
        )))
        .with_block(Block::Section(SectionBlock::with_text(
            TextObject::markdown(calendar_day.name_days.join(", ")),
        )))
        .with_block(Block::Divider)
        .with_block(Block::Section(SectionBlock::with_fields(vec![
            TextObject::markdown("*Title:*\nText".to_owned()),
            TextObject::markdown("*Title 2:*\nText 2".to_owned()),
        ])));
    client.send(&message).expect("Could not send slack message");
}
