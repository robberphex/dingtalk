#[macro_use]
extern crate json;

use std::{
    fs,
    env,
    path::PathBuf,
    time::SystemTime,
    io::{
        Error,
        ErrorKind,
    },
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
#[derive(Default)]
pub struct DingTalk<'a> {
    pub default_webhook_url: &'a str,
    pub access_token: &'a str,
    pub sec_token: &'a str,
    pub direct_url: &'a str,
}

/// DingTalk message type
/// * Text - text message
/// * Markdown - markdown message
/// * Link - link message
/// * ActionCard - action card message
/// * FeedCard - feed card message
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DingTalkMessageType {
    Text,
    Markdown,
    Link,
    ActionCard,
    FeedCard,
}

/// Default DingTalkMessageType is Text
impl Default for DingTalkMessageType {
    fn default() -> Self { DingTalkMessageType::Text }
}

/// DingTalk messge action card avatar
#[derive(Clone, Copy, Debug)]
pub enum DingTalkMessageActionCardHideAvatar {
    Hide,
    Show,
}

// default value
impl Default for DingTalkMessageActionCardHideAvatar {
    fn default() -> Self { DingTalkMessageActionCardHideAvatar::Show }
}

/// into JsonValue
impl From<DingTalkMessageActionCardHideAvatar> for json::JsonValue {
    fn from(a: DingTalkMessageActionCardHideAvatar) -> Self {
        json::JsonValue::String(match a {
            DingTalkMessageActionCardHideAvatar::Show => "0".into(),
            DingTalkMessageActionCardHideAvatar::Hide => "1".into(),
        })
    }
}

/// DingTalk message action card orientation
#[derive(Clone, Copy, Debug)]
pub enum DingTalkMessageActionCardBtnOrientation {
    Vertical,
    Landscape,
}

/// default value
impl Default for DingTalkMessageActionCardBtnOrientation {
    fn default() -> Self { DingTalkMessageActionCardBtnOrientation::Vertical }
}

/// into JsonValue
impl From<DingTalkMessageActionCardBtnOrientation> for json::JsonValue {
    fn from(o: DingTalkMessageActionCardBtnOrientation) -> Self {
        json::JsonValue::String(match o {
            DingTalkMessageActionCardBtnOrientation::Vertical => "0".into(),
            DingTalkMessageActionCardBtnOrientation::Landscape => "1".into(),
        })
    }
}

/// DingTalk message action card btn
#[derive(Debug)]
pub struct DingTalkMessageActionCardBtn {
    pub title: String,
    pub action_url: String,
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
    pub action_card_title: &'a str,
    pub action_card_text: &'a str,
    pub action_card_hide_avatar: DingTalkMessageActionCardHideAvatar,
    pub action_card_btn_orientation: DingTalkMessageActionCardBtnOrientation,
    pub action_card_single_btn: Option<DingTalkMessageActionCardBtn>,
    pub action_card_btns: Vec<DingTalkMessageActionCardBtn>,
    pub feed_card_links: Vec<DingTalkMessageFeedCardLink>,
    pub at_all: bool,
    pub at_mobiles: Vec<String>,
}

impl <'a> DingTalkMessage<'a> {

    /// New text DingTalk message
    pub fn new_text(text_content: &'a str) -> Self {
        Self::new(DingTalkMessageType::Text).text(text_content)
    }

    /// New markdown DingTalk message
    pub fn new_markdown(markdown_title: &'a str, markdown_content: &'a str) -> Self {
        Self::new(DingTalkMessageType::Markdown).markdown(markdown_title, markdown_content)
    }

    /// New link DingTalk message
    pub fn new_link(link_title: &'a str, link_text: &'a str, link_pic_url: &'a str, link_message_url: &'a str) -> Self {
        Self::new(DingTalkMessageType::Link).link(link_title, link_text, link_pic_url, link_message_url)
    }

    /// New action card DingTalk message
    pub fn new_action_card(title: &'a str, text: &'a str) -> Self {
        let mut s = Self::new(DingTalkMessageType::ActionCard);
        s.action_card_title = title;
        s.action_card_text = text;
        s
    }

    /// New feed card DingTalk message
    pub fn new_feed_card() -> Self {
        Self::new(DingTalkMessageType::FeedCard)
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

    /// Set action card show avator(default show)
    pub fn action_card_show_avatar(mut self) -> Self {
        self.action_card_hide_avatar = DingTalkMessageActionCardHideAvatar::Show;
        self
    }

    /// Set action card hide avator
    pub fn action_card_hide_avatar(mut self) -> Self {
        self.action_card_hide_avatar = DingTalkMessageActionCardHideAvatar::Hide;
        self
    }

    /// Set action card btn vertical(default vertical)
    pub fn action_card_btn_vertical(mut self) -> Self {
        self.action_card_btn_orientation = DingTalkMessageActionCardBtnOrientation::Vertical;
        self
    }

    /// Set action card btn landscape
    pub fn action_card_btn_landscape(mut self) -> Self {
        self.action_card_btn_orientation = DingTalkMessageActionCardBtnOrientation::Landscape;
        self
    }

    /// Set action card single btn
    pub fn set_action_card_signle_btn(mut self, btn: DingTalkMessageActionCardBtn) -> Self {
        self.action_card_single_btn = Some(btn);
        self
    }

    /// Add action card btn
    pub fn add_action_card_btn(mut self, btn: DingTalkMessageActionCardBtn) -> Self {
        self.action_card_btns.push(btn);
        self
    }
    
    /// Add feed card link
    pub fn add_feed_card_link(mut self, link: DingTalkMessageFeedCardLink) -> Self {
        self.feed_card_links.push(link);
        self
    }

    /// Add feed card link detail
    pub fn add_feed_card_link_detail(self, title: &'a str, message_url: &'a str, pic_url: &'a str) -> Self {
        self.add_feed_card_link(DingTalkMessageFeedCardLink {
            title: title.to_owned(),
            message_url: message_url.to_owned(),
            pic_url: pic_url.to_owned(),
        })
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
            ..Default::default()
        })
    }

    /// Create `DingTalk` from url, for outgoing robot
    pub fn from_url(url: &'a str) -> Self {
        DingTalk {
            direct_url: url,
            ..Default::default()
        }
    }

    /// Create `DingTalk`
    /// `access_token` is access token, `sec_token` can be empty `""`
    pub fn new(access_token: &'a str, sec_token: &'a str) -> Self {
        DingTalk {
            default_webhook_url: DEFAULT_DINGTALK_ROBOT_URL,
            access_token: access_token,
            sec_token: sec_token,
            ..Default::default()
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
    pub async fn send_message(&self, dingtalk_message: &DingTalkMessage<'_>) -> XResult<()> {
        let mut message_json = match dingtalk_message.message_type {
            DingTalkMessageType::Text => object!{
                "msgtype" => "text",
                "text" => object! {
                    "content" => dingtalk_message.text_content,
                }
            },
            DingTalkMessageType::Link => object!{
                "msgtype" => "link",
                "link" => object!{
                    "text" => dingtalk_message.link_text,
                    "title" => dingtalk_message.link_title,
                    "picUrl" => dingtalk_message.link_pic_url,
                    "messageUrl" => dingtalk_message.link_message_url,
                }
            },
            DingTalkMessageType::Markdown => object!{
                "msgtype" => "markdown",
                "markdown" => object! {
                    "title" => dingtalk_message.markdown_title,
                    "text" => dingtalk_message.markdown_content,
                }
            },
            DingTalkMessageType::ActionCard => object!{
                "msgtype" => "actionCard",
                "actionCard" => object!{
                    "title" => dingtalk_message.action_card_title,
                    "text" => dingtalk_message.action_card_text,
                    "hideAvatar" => dingtalk_message.action_card_hide_avatar,
                    "btnOrientation" => dingtalk_message.action_card_btn_orientation,
                },
            },
            DingTalkMessageType::FeedCard => object!{
                "msgtype" => "feedCard",
            },
        };
        if DingTalkMessageType::ActionCard == dingtalk_message.message_type {
            if dingtalk_message.action_card_single_btn.is_some() {
                let single_btn = dingtalk_message.action_card_single_btn.as_ref().unwrap();
                message_json["actionCard"]["singleTitle"] = single_btn.title.as_str().into();
                message_json["actionCard"]["singleURL"] = single_btn.action_url.as_str().into();
            } else {
                let mut btns: Vec<json::JsonValue> = vec![];
                for action_card_btn in &dingtalk_message.action_card_btns {
                    let btn = object!{
                        "title" => action_card_btn.title.as_str(),
                        "actionURL" => action_card_btn.action_url.as_str(),
                    };
                    btns.push(btn);
                }
                message_json["actionCard"]["btns"] = json::JsonValue::Array(btns);
            }
        }
        if DingTalkMessageType::FeedCard == dingtalk_message.message_type {
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
        self.send(&json::stringify(message_json)).await
    }

    /// Send text message
    pub async fn send_text(&self, text_message: &str) -> XResult<()> {
        self.send_message(&DingTalkMessage::new_text(text_message)).await
    }

    /// Send markdown message
    pub async fn send_markdown(&self, title: &str, text: &str) -> XResult<()> {
        self.send_message(&DingTalkMessage::new_markdown(title, text)).await
    }

    /// Send link message
    pub async fn send_link(&self, link_title: &'a str, link_text: &'a str, link_pic_url: &'a str, link_message_url: &'a str) -> XResult<()> {
        self.send_message(&DingTalkMessage::new_link(link_title, link_text, link_pic_url, link_message_url)).await
    }

    /// Direct send JSON message
    pub async fn send(&self, json_message: &str) -> XResult<()> {
        let client = reqwest::Client::new();
        let response = match client.post(&self.generate_signed_url())
              .header(CONTENT_TYPE, APPLICATION_JSON_UTF8)
              .body(json_message.as_bytes().to_vec())
              .send().await {
                  Ok(r) => r,
                  Err(e) => return Err(Box::new(Error::new(ErrorKind::Other, format!("Unknown error: {}", e))) as Box<dyn std::error::Error>),
              };

        match response.status().as_u16() {
            200_u16 => Ok(()),
            _ => Err(Box::new(Error::new(ErrorKind::Other, format!("Unknown status: {}", response.status().as_u16()))) as Box<dyn std::error::Error>),
        }
    }

    /// Generate signed dingtalk webhook URL
    pub fn generate_signed_url(&self) -> String {
        if !self.direct_url.is_empty() {
            return self.direct_url.into();
        }
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

#[test]
fn run_all_tests() {
    tokio_test::block_on(_test_send()).unwrap();
}

async fn _test_send() -> XResult<()> {
    let dt = DingTalk::from_file("~/.dingtalk-token.json")?;
    dt.send_text("test message 001 ---------------------").await?;

    dt.send_markdown("markdown title 001", r#"# markdown content 001
* line 0
* line 1
* line 2"#).await?;

    dt.send_link("link title 001", "link content 001", "https://hatter.ink/favicon.png", "https://hatter.ink/").await?;

    dt.send_message(&DingTalkMessage::new_feed_card()
        .add_feed_card_link(DingTalkMessageFeedCardLink{
            title: "test feed card title 001".into(),
            message_url: "https://hatter.ink/".into(),
            pic_url: "https://hatter.ink/favicon.png".into(),
        })
        .add_feed_card_link(DingTalkMessageFeedCardLink{
            title: "test feed card title 002".into(),
            message_url: "https://hatter.ink/".into(),
            pic_url: "https://hatter.ink/favicon.png".into(),
        })
    ).await?;

    dt.send_message(&DingTalkMessage::new_action_card("action card 001", "action card text 001")
        .set_action_card_signle_btn(DingTalkMessageActionCardBtn{
            title: "test signle btn title".into(),
            action_url: "https://hatter.ink/".into(),
        })
    ).await?;

    dt.send_message(&DingTalkMessage::new_action_card("action card 002", "action card text 002")
        .add_action_card_btn(DingTalkMessageActionCardBtn{
            title: "test signle btn title 01".into(),
            action_url: "https://hatter.ink/".into(),
        })
        .add_action_card_btn(DingTalkMessageActionCardBtn{
            title: "test signle btn title 02".into(),
            action_url: "https://hatter.ink/".into(),
        })
    ).await?;

    Ok(())
}
