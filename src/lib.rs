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

pub struct DingTalk<'a> {
    pub access_token: &'a str,
    pub sec_token: &'a str,
}

impl <'a> DingTalk<'a> {

    pub fn new(access_token: &'a str, sec_token: &'a str) -> Self {
        DingTalk {
            access_token: access_token,
            sec_token: sec_token,
        }
    }

    pub fn send_text(&self, text_message: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.send(&json::stringify(object!{
            "msgtype" => "text",
            "text" => object! {
                "content" => text_message,
            }
        }))
    }

    pub fn send_markdown(&self, title: &str, text: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.send(&json::stringify(object!{
            "msgtype" => "markdown",
            "markdown" => object! {
                "title" => title,
                "text" => text,
            }
        }))
    }

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

    pub fn generate_signed_url(&self) -> String {
        let mut signed_url = String::with_capacity(1024);
        signed_url.push_str(DEFAULT_DINGTALK_ROBOT_URL);
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


fn calc_hmac_sha256(key: &[u8], message: &[u8]) -> MacResult {
    let mut hmac = Hmac::new(Sha256::new(), key);
    hmac.input(message);
    hmac.result()
}
