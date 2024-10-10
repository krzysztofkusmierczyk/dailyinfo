use serde::Serialize;

pub struct SlackWebhookUrl(String);

impl SlackWebhookUrl {
    pub fn new(url: String) -> Self {
        Self(url)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Serialize, Debug)]
pub enum TextType {
    #[serde(rename = "mrkdwn")]
    Mrkdwn,
    #[serde(rename = "plain_text")]
    PlainText,
}

#[derive(Serialize, Debug)]
pub struct TextObject {
    #[serde(rename = "type")]
    pub object_type: TextType,
    pub text: String,
}

impl TextObject {
    pub fn plain(text: String) -> Self {
        Self {
            object_type: TextType::PlainText,
            text,
        }
    }

    pub fn markdown(text: String) -> Self {
        Self {
            object_type: TextType::Mrkdwn,
            text,
        }
    }
}

#[derive(Serialize, Debug)]
#[serde(tag = "type", content = "text")]
pub enum Block {
    #[serde(rename = "header")]
    Header(TextObject),
}

#[derive(Serialize, Debug, Default)]
pub struct Message {
    pub text: String,
    pub blocks: Option<Vec<Block>>,
}

impl Message {
    pub fn new(text: String) -> Self {
        Self { text, blocks: None }
    }

    pub fn with_blocks(mut self, blocks: Vec<Block>) -> Message {
        self.blocks = Some(blocks);
        self
    }

    pub fn push_block(mut self, block: Block) {
        match self.blocks {
            Some(mut bl) => bl.push(block),
            None => self.blocks = Some(vec![block]),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use serde_json;

    #[test]
    fn serializes_plain_text_object() {
        let text_object = TextObject::plain("Test text".to_string());

        let result = serde_json::to_string(&text_object);
        assert_eq!(
            result.unwrap(),
            r#"{"type":"plain_text","text":"Test text"}"#
        );
    }

    #[test]
    fn serializes_mrkdwn_text_object() {
        let text_object = TextObject::markdown("Test text".to_string());

        let result = serde_json::to_string(&text_object);
        assert_eq!(result.unwrap(), r#"{"type":"mrkdwn","text":"Test text"}"#);
    }

    #[test]
    fn serializes_header_block() {
        let header_block = Block::Header(TextObject::plain("Test text".to_string()));

        let result = serde_json::to_string(&header_block);
        assert_eq!(
            result.unwrap(),
            r#"{"type":"header","text":{"type":"plain_text","text":"Test text"}}"#
        );
    }
}
