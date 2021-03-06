//! This module contains Structs that can be serialized
//! to send parameters to the telegram servers using the Bot class.

extern crate serde_json;
extern crate tg_bot_models;

use self::tg_bot_models::*;


/// This struct contains all parameters available for the send method. It directly serializes to
/// JSON and offers a builder pattern to configure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageParams {
    chat_id: String,
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_web_page_preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<KeyboardDef>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum KeyboardDef {
    Writing(ReplyKeyboardRemove),
    Custom(ReplyKeyboardMarkup),
    MessageButtons(InlineKeyboardMarkup),
    Reply(ForceReply),
}


impl MessageParams {
    // TODO Should these methods offer to reset values to None?
    pub fn new(chat_id: String, text: String) -> Self {
        MessageParams {
            chat_id,
            text,
            parse_mode: Some("Markdown".into()),
            disable_web_page_preview: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    pub fn as_html<'a>(&'a mut self) -> &'a mut Self {
        self.parse_mode = Some("HTML".to_owned());
        self
    }

    pub fn as_markdown<'a>(&'a mut self) -> &'a mut Self {
        self.parse_mode = Some("Markdown".to_owned());
        self
    }

    pub fn hide_link_preview<'a>(&'a mut self, preview: bool) -> &'a mut Self {
        self.disable_web_page_preview = Some(preview);
        self
    }

    pub fn hide_notification<'a>(&'a mut self, notify: bool) -> &'a mut Self {
        self.disable_notification = Some(notify);
        self
    }

    pub fn reply_to_message_id<'a>(&'a mut self, message_id: i64) -> &'a mut Self {
        self.reply_to_message_id = Some(message_id);
        self
    }

    pub fn reply_to_message<'a>(&'a mut self, message: &Message) -> &'a mut Self {
        self.reply_to_message_id = Some(message.message_id);
        self
    }

    pub fn set_keyboard<'a>(&'a mut self, keyboard: KeyboardDef) -> &'a mut Self {
        self.reply_markup = Some(keyboard);
        self
    }

    pub fn build(&self) -> Self {
        self.clone()
    }
}
