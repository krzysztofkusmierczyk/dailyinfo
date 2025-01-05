use crate::providers::{calendar::get_calendar_day, greeter::greet};

use crate::slack::{
    Block, HeaderBlock, Message, SectionBlock, SlackWebhookClient, SlackWebhookUrl, TextObject,
};
use chrono::Utc;
pub fn send_message<S: Into<String>>(webhook_url: S) {
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
        .with_block(Block::Section(SectionBlock::with_text(
            TextObject::markdown("*Święta*"),
        )))
        .with_block(Block::Section(SectionBlock::with_text(
            TextObject::markdown(
                calendar_day
                    .festivities
                    .into_iter()
                    .map(|f| format!("- {}", f))
                    .collect::<Vec<_>>()
                    .join("\n"),
            ),
        )));
    client.send(&message).expect("Could not send slack message");
}
