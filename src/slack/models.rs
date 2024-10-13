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
pub struct HeaderBlock {
    text: TextObject,
}

impl HeaderBlock {
    pub fn with_text(text: TextObject) -> Self {
        Self { text }
    }
}

#[derive(Serialize, Debug)]
pub struct SectionBlock {
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<TextObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fields: Option<Vec<TextObject>>,
}

impl SectionBlock {
    pub fn with_text(text: TextObject) -> Self {
        Self {
            text: Some(text),
            fields: None,
        }
    }

    pub fn with_fields(fields: Vec<TextObject>) -> Self {
        Self {
            text: None,
            fields: Some(fields),
        }
    }
}

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub enum Block {
    #[serde(rename = "header")]
    Header(HeaderBlock),
    #[serde(rename = "divider")]
    Divider,
    #[serde(rename = "section")]
    Section(SectionBlock),
}

#[derive(Serialize, Debug, Default)]
pub struct Message {
    pub text: Option<String>,
    pub blocks: Option<Vec<Block>>,
}

impl Message {
    pub fn new(text: String) -> Self {
        Self {
            text: Some(text),
            blocks: None,
        }
    }

    pub fn with_blocks(&mut self, blocks: Vec<Block>) {
        self.blocks = Some(blocks);
    }

    pub fn push_block(&mut self, block: Block) {
        match &mut self.blocks {
            Some(bl) => bl.push(block),
            None => self.blocks = Some(vec![block]),
        }
    }

    pub fn with_block(&mut self, block: Block) -> &mut Self {
        self.push_block(block);
        self
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
        let header_block = Block::Header(HeaderBlock::with_text(TextObject::plain(
            "Test text".to_string(),
        )));

        let result = serde_json::to_string(&header_block);
        assert_eq!(
            result.unwrap(),
            r#"{"type":"header","text":{"type":"plain_text","text":"Test text"}}"#
        );
    }

    #[test]
    fn serializes_divider_block() {
        let divider_block = Block::Divider;

        let result = serde_json::to_string(&divider_block);

        assert_eq!(result.unwrap(), r#"{"type":"divider"}"#);
    }

    #[test]
    fn serializes_section_block_with_text() {
        let section = Block::Section(SectionBlock::with_text(TextObject::plain(
            "Test text".to_owned(),
        )));

        let result = serde_json::to_string(&section);

        assert_eq!(
            result.unwrap(),
            r#"{"type":"section","text":{"type":"plain_text","text":"Test text"}}"#
        );
    }

    #[test]
    fn serializes_section_block_with_fields() {
        let section = Block::Section(SectionBlock::with_fields(vec![TextObject::plain(
            "Test text".to_owned(),
        )]));

        let result = serde_json::to_string(&section);

        assert_eq!(
            result.unwrap(),
            r#"{"type":"section","fields":[{"type":"plain_text","text":"Test text"}]}"#
        );
    }
    #[test]
    fn serializes_comples_message() {
        let mut message = Message::new("Notification text".to_owned());
        message.push_block(Block::Header(HeaderBlock::with_text(TextObject::plain(
            "Headet text".to_owned(),
        ))));
        message.push_block(Block::Divider);
        message.push_block(Block::Section(SectionBlock::with_text(
            TextObject::markdown("Markdown text\nand another!".to_owned()),
        )));
        message.push_block(Block::Divider);
        message.push_block(Block::Section(SectionBlock::with_fields(vec![
            TextObject::markdown("*Title:*\nText".to_owned()),
            TextObject::markdown("*Title 2:*\nText 2".to_owned()),
        ])));

        let result = serde_json::to_string(&message);

        assert_eq!(
            result.unwrap(),
            r#"{"text":"Notification text","blocks":[{"type":"header","text":{"type":"plain_text","text":"Headet text"}},{"type":"divider"},{"type":"section","text":{"type":"mrkdwn","text":"Markdown text\nand another!"}},{"type":"divider"},{"type":"section","fields":[{"type":"mrkdwn","text":"*Title:*\nText"},{"type":"mrkdwn","text":"*Title 2:*\nText 2"}]}]}"#
        );
    }
}
