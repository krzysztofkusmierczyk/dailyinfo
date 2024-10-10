use crate::slack::error::Result;
use crate::slack::models::{Message, SlackWebhookUrl};
use ureq;

pub struct SlackWebhookClient {
    url: SlackWebhookUrl,
}

impl SlackWebhookClient {
    pub fn new(url: SlackWebhookUrl) -> Self {
        return SlackWebhookClient { url };
    }

    pub fn send(self, message: &Message) -> Result<()> {
        ureq::post(self.url.as_str()).send_json(message)?;

        Ok(())
    }
}
