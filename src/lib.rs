#[macro_use]
extern crate json;

use std::time::SystemTime;
use std::io::{
    Error,
    ErrorKind,
};
use crypto::{
    mac::{
        Mac,
        MacResult,
    },
    hmac::Hmac,
    sha2::Sha256,
};

const CONTENT_TYPE: &str = "Content-Type";
const APPLICATION_JSON_UTF8: &str = "application/json; charset=utf-8";

const DEFAULT_DINGTALK_ROBOT_URL: &str = "https://oapi.dingtalk.com/robot/send?access_token=";

/// `DingTalk` is a simple SDK for DingTalk webhook robot
/// 
/// Document https://ding-doc.dingtalk.com/doc#/serverapi2/qf2nxq
/// 
/// Sample code:
/// ```
/// let dt = DingTalk::new("<token>", "");
/// dt.send_text("Hello world!")?;
/// ```
/// 
/// At all sample:
/// ```
/// dt.send_message(&DingTalkMessage::new_text("Hello World!").at_all())?;
/// ```
pub struct DingTalk<'a> {
    pub default_webhook_url: &'a str,
    pub access_token: &'a str,
    pub sec_token: &'a str,
}

/// DingTalk message type
/// * TEXT - text message
/// * MARKDONW - markdown message
#[derive(Clone, Copy, Debug)]
pub enum DingTalkMessageType {
    TEXT,
    LINK,
    MARKDOWN,
    // ACTION_CARD, todo!()
}

/// Default DingTalkMessageType is TEXT
impl Default for DingTalkMessageType {

    fn default() -> Self { DingTalkMessageType::TEXT }
}

/// DingTalk message
#[derive(Debug, Default)]
pub struct DingTalkMessage<'a> {
    pub message_type: DingTalkMessageType,
    pub text_content: &'a str,
    pub markdown_title: &'a str,
    pub markdown_content: &'a str,
    pub link_text: &'a str,
    pub link_title: &'a str,
    pub link_pic_url: &'a str,
    pub link_message_url: &'a str,
    pub at_all: bool,
    pub at_mobiles: Vec<String>,
}

impl <'a> DingTalkMessage<'a> {

    /// New text DingTalk message
    pub fn new_text(text_content: &'a str) -> Self {
        Self::new(DingTalkMessageType::TEXT).text(text_content)
    }

    /// New markdown DingTalk message
    pub fn new_markdown(markdown_title: &'a str, markdown_content: &'a str) -> Self {
        Self::new(DingTalkMessageType::MARKDOWN).markdown(markdown_title, markdown_content)
    }

    /// New link DingTalk message
    pub fn new_link(link_title: &'a str, link_text: &'a str, link_pic_url: &'a str, link_message_url: &'a str) -> Self {
        Self::new(DingTalkMessageType::LINK).link(link_title, link_text, link_pic_url, link_message_url)
    }
    
    /// New DingTalk message
    pub fn new(message_type: DingTalkMessageType) -> Self {
        DingTalkMessage {
            message_type: message_type,
            ..Default::default()
        }
    }

    /// Set text
    pub fn text(mut self, text_content: &'a str) -> Self {
        self.text_content = text_content;
        self
    }

    /// Set link
    pub fn link(mut self, link_title: &'a str, link_text: &'a str, link_pic_url: &'a str, link_message_url: &'a str) -> Self {
        self.link_title = link_title;
        self.link_text = link_text;
        self.link_pic_url = link_pic_url;
        self.link_message_url = link_message_url;
        self
    }

    /// Set markdown
    pub fn markdown(mut self, markdown_title: &'a str, markdown_content: &'a str) -> Self {
        self.markdown_title = markdown_title;
        self.markdown_content = markdown_content;
        self
    }

    // At all
    pub fn at_all(mut self) -> Self {
        self.at_all = true;
        self
    }

    // At mobiles
    pub fn at_mobiles(mut self, mobiles: &Vec<String>) -> Self {
        for m in mobiles {
            self.at_mobiles.push(m.clone());
        }
        self
    }
}

impl <'a> DingTalk<'a> {

    /// Create `DingTalk`
    /// `access_token` is access token, `sec_token` can be empty `""`
    pub fn new(access_token: &'a str, sec_token: &'a str) -> Self {
        DingTalk {
            default_webhook_url: DEFAULT_DINGTALK_ROBOT_URL,
            access_token: access_token,
            sec_token: sec_token,
        }
    }

    /// Set default webhook url
    pub fn set_default_webhook_url(&mut self, default_webhook_url: &'a str) {
        self.default_webhook_url = default_webhook_url;
    }

    /// Send DingTalk message
    pub fn send_message(&self, dingtalk_message: &DingTalkMessage) -> Result<(), Box<dyn std::error::Error>> {
        let mut message_json = match dingtalk_message.message_type {
            DingTalkMessageType::TEXT => object!{
                "msgtype" => "text",
                "text" => object! {
                    "content" => dingtalk_message.text_content,
                }
            },
            DingTalkMessageType::LINK => object!{
                "msgtype" => "link",
                "link" => object!{
                    "text" => dingtalk_message.link_text,
                    "title" => dingtalk_message.link_title,
                    "picUrl" => dingtalk_message.link_pic_url,
                    "messageUrl" => dingtalk_message.link_message_url,
                }
            },
            DingTalkMessageType::MARKDOWN => object!{
                "msgtype" => "markdown",
                "markdown" => object! {
                    "title" => dingtalk_message.markdown_title,
                    "text" => dingtalk_message.markdown_content,
                }
            },
        };
        if dingtalk_message.at_all || dingtalk_message.at_mobiles.len() > 0 {
            let mut at_mobiles = json::JsonValue::new_object();
            for m in &dingtalk_message.at_mobiles {
                at_mobiles.push(m.clone()).ok();
            }
            message_json["at"] = object!{
                "atMobiles" => at_mobiles,
                "isAtAll" => dingtalk_message.at_all,
            };
        }
        self.send(&json::stringify(message_json))
    }

    /// Send text message
    pub fn send_text(&self, text_message: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.send_message(&DingTalkMessage::new_text(text_message))
    }

    /// Send markdown message
    pub fn send_markdown(&self, title: &str, text: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.send_message(&DingTalkMessage::new_markdown(title, text))
    }

    /// Send link message
    pub fn send_link(&self, link_title: &'a str, link_text: &'a str, link_pic_url: &'a str, link_message_url: &'a str) -> Result<(), Box<dyn std::error::Error>> {
        self.send_message(&DingTalkMessage::new_link(link_title, link_text, link_pic_url, link_message_url))
    }

    /// Direct send JSON message
    pub fn send(&self, json_message: &str) -> Result<(), Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let response = client.post(&self.generate_signed_url())
              .header(CONTENT_TYPE, APPLICATION_JSON_UTF8)
              .body(json_message.as_bytes().to_vec())
              .send()?;

        match response.status().as_u16() {
            200_u16 => Ok(()),
            _ => Err(Box::new(Error::new(ErrorKind::Other, format!("Unknown status: {}", response.status().as_u16())))),
        }
    }

    /// Generate signed dingtalk webhook URL
    pub fn generate_signed_url(&self) -> String {
        let mut signed_url = String::with_capacity(1024);
        signed_url.push_str(self.default_webhook_url);
        signed_url.push_str(&urlencoding::encode(self.access_token));

        if self.sec_token != "" {
            let timestamp = &format!("{}", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis());
            let timestamp_and_secret = &format!("{}\n{}", timestamp, self.sec_token);
            let hmac_sha256 = base64::encode(calc_hmac_sha256(self.sec_token.as_bytes(), timestamp_and_secret.as_bytes()).code());

            signed_url.push_str("&timestamp=");
            signed_url.push_str(timestamp);
            signed_url.push_str("&sign=");
            signed_url.push_str(&urlencoding::encode(&hmac_sha256));
        }
        
        signed_url
    }
}

/// calc hma_sha256 digest
fn calc_hmac_sha256(key: &[u8], message: &[u8]) -> MacResult {
    let mut hmac = Hmac::new(Sha256::new(), key);
    hmac.input(message);
    hmac.result()
}
