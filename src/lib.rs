#[macro_use]
extern crate json;

use std::fs;
use std::env;
use std::path::PathBuf;
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

pub type XResult<T> = Result<T, Box<dyn std::error::Error>>;

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
/// * LINK - link message
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DingTalkMessageType {
    TEXT,
    LINK,
    MARKDOWN,
    // ACTION_CARD, todo!()
    FEEDCARD,
}

/// Default DingTalkMessageType is TEXT
impl Default for DingTalkMessageType {

    fn default() -> Self { DingTalkMessageType::TEXT }
}

/// DingTalk message feed card link
#[derive(Debug)]
pub struct DingTalkMessageFeedCardLink {
    pub title: String,
    pub message_url: String,
    pub pic_url: String,
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
    pub feed_card_links: Vec<DingTalkMessageFeedCardLink>,
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

    /// Set markdown
    pub fn markdown(mut self, markdown_title: &'a str, markdown_content: &'a str) -> Self {
        self.markdown_title = markdown_title;
        self.markdown_content = markdown_content;
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

    /// At all
    pub fn at_all(mut self) -> Self {
        self.at_all = true;
        self
    }

    /// At mobiles
    pub fn at_mobiles(mut self, mobiles: &Vec<String>) -> Self {
        for m in mobiles {
            self.at_mobiles.push(m.clone());
        }
        self
    }
}

impl <'a> DingTalk<'a> {

    /// Create `DingTalk` from file
    /// 
    /// Format see `DingTalk::from_json(json: &str)`
    pub fn from_file(f: &str) -> XResult<Self> {
        let f_path_buf = if f.starts_with("~/") {
            let home = PathBuf::from(env::var("HOME")?);
            home.join(f.chars().skip(2).collect::<String>())
        } else {
            PathBuf::from(f)
        };
        let f_content = fs::read_to_string(f_path_buf)?;
        Self::from_json(&f_content)
    }

    /// Create `DingTalk` from JSON string
    /// 
    /// Format:
    /// ```json
    /// {
    ///     "default_webhook_url": "", // option
    ///     "access_token": "<access token>",
    ///     "sec_token": "<sec token>" // option
    /// }
    /// ```
    pub fn from_json(json: &str) -> XResult<Self> {
        let json_value = json::parse(json)?;
        if !json_value.is_object() {
            return Err(Box::new(Error::new(ErrorKind::Other, format!("JSON format erorr: {}", json))));
        }

        let default_webhook_url = Self::string_to_a_str(json_value["default_webhook_url"].as_str().unwrap_or(DEFAULT_DINGTALK_ROBOT_URL));
        let access_token = Self::string_to_a_str(json_value["access_token"].as_str().unwrap_or_default());
        let sec_token = Self::string_to_a_str(json_value["sec_token"].as_str().unwrap_or_default());
        
        Ok(DingTalk {
            default_webhook_url: default_webhook_url,
            access_token: access_token,
            sec_token: sec_token,
        })
    }

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
    /// 
    /// 1. Create DingTalk JSON message
    /// 2. POST JSON message to DingTalk server
    pub fn send_message(&self, dingtalk_message: &DingTalkMessage) -> XResult<()> {
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
            DingTalkMessageType::FEEDCARD => object!{
                "msgtype" => "feedCard",
            },
        };
        if DingTalkMessageType::FEEDCARD == dingtalk_message.message_type {
            let mut links: Vec<json::JsonValue> = vec![];
            for feed_card_link in &dingtalk_message.feed_card_links {
                let link = object!{
                    "title" => feed_card_link.title.as_str(),
                    "messageURL" => feed_card_link.message_url.as_str(),
                    "picURL" => feed_card_link.pic_url.as_str(),
                };
                links.push(link);
            }
            message_json["feedCard"] = object!{
                "links" => json::JsonValue::Array(links),
            };
        }
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
    pub fn send_text(&self, text_message: &str) -> XResult<()> {
        self.send_message(&DingTalkMessage::new_text(text_message))
    }

    /// Send markdown message
    pub fn send_markdown(&self, title: &str, text: &str) -> XResult<()> {
        self.send_message(&DingTalkMessage::new_markdown(title, text))
    }

    /// Send link message
    pub fn send_link(&self, link_title: &'a str, link_text: &'a str, link_pic_url: &'a str, link_message_url: &'a str) -> XResult<()> {
        self.send_message(&DingTalkMessage::new_link(link_title, link_text, link_pic_url, link_message_url))
    }

    /// Direct send JSON message
    pub fn send(&self, json_message: &str) -> XResult<()> {
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

    // SAFE? may these codes cause memory leak?
    fn string_to_a_str(s: &str) -> &'a str {
        Box::leak(s.to_owned().into_boxed_str())
    }
}

/// calc hma_sha256 digest
fn calc_hmac_sha256(key: &[u8], message: &[u8]) -> MacResult {
    let mut hmac = Hmac::new(Sha256::new(), key);
    hmac.input(message);
    hmac.result()
}
