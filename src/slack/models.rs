use serde::Serialize;

pub struct SlackWebhookUrl(String);

impl SlackWebhookUrl {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Serialize, Debug)]
struct Field {
    pub title: String,
    pub value: String,
}

impl Field {
    pub fn new(title: String, value: String) -> Self {
        Self { title, value }
    }
}

#[derive(Serialize, Debug)]
pub struct Attachment {
    pub text: String,
    pub fields: Vec<Field>,
}

impl Attachment {
    pub fn new(text: String, fields: Vec<Field>) -> Self {
        Self { text, fields }
    }
}

#[derive(Serialize, Debug, Default)]
pub struct Message {
    pub attachments: Vec<Attachment>,
}

impl Message {
    pub fn with_attachment(attachment: Attachment) -> Self {
        Self {
            attachments: vec![attachment],
        }
    }

    pub fn with_attachments(attachments: Vec<Attachment>) -> Message {
        return Message { attachments };
    }

    pub fn add_attachment(&mut self, attachment: Attachment) {
        self.attachments.push(attachment);
    }
}
