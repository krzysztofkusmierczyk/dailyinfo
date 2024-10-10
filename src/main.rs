use dotenvy::dotenv;
use slack::{Attachment, Message, SlackWebhookClient, SlackWebhookUrl};
use std::env;
mod slack;

fn main() {
    dotenv().ok();

    let webhook_url = env::var("SLACK_WEBHOOK").expect("SLACK_WEBHOOK not set");

    let client = SlackWebhookClient::new(SlackWebhookUrl::new(webhook_url));
    // TODO: use blocks instead of attachments https://app.slack.com/block-kit-builder/
    let message = Message::with_attachments(vec![Attachment {
        text: "Hello world".to_string(),
        fields: vec![],
    }]);
    client.send(&message).expect("Could not send slack message");
}
