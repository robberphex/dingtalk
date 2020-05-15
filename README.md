# dingtalk

DingTalk Robot Util, Send text/markdown/link messages using DingTalk robot

钉钉机器人 Rust SDK

NOTE: From version 1.1.0 dingtalk uses reqwest 0.10.0's `async`/`.await` API.

> Official reference: https://ding-doc.dingtalk.com/doc#/serverapi2/qf2nxq/0fa88adc


Sample 1:
```rust
use dingtalk::DingTalk;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dt = DingTalk::new("<token>", "");
    dt.send_text("Hello world!").await?;

    Ok(())
}
```

Need use crate: `tokio = { version = "0.2.6", features = ["full"] }`.

Sample 2 (Read token from file):
```rust
#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dt = DingTalk::from_file("~/.dingtalk-token.json")?;
    dt.send_text("Hello world!").await?;

    Ok(())
}
```

Sample, send markdown message:
```rust
dt.send_markdown("markdown title 001", r#"# markdown content 001
* line 0
* line 1
* line 2"#).await?;
```

Sample, send link message:
```rust
dt.send_link("link title 001", "link content 001", "https://hatter.ink/favicon.png", "https://hatter.ink/").await?;
```

Sample, send feed card message:
```rust
dt.send_message(DingTalkMessage::new_feed_card()
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
```

Sample, send action card message(single btn):
```rust
dt.send_message(DingTalkMessage::new_action_card("action card 001", "action card text 001")
    .set_action_card_signle_btn(DingTalkMessageActionCardBtn{
        title: "test signle btn title".into(),
        action_url: "https://hatter.ink/".into(),
    })
).await?;
```

Sample, send action card message(multi btns):
```rust
dt.send_message(DingTalkMessage::new_action_card("action card 002", "action card text 002")
    .add_action_card_btn(DingTalkMessageActionCardBtn{
        title: "test signle btn title 01".into(),
        action_url: "https://hatter.ink/".into(),
    })
    .add_action_card_btn(DingTalkMessageActionCardBtn{
        title: "test signle btn title 02".into(),
        action_url: "https://hatter.ink/".into(),
    })
).await?;
```

#### JSON Config

DingTalk config:
```json
{
  "access_token": "<access token>",
  "sec_token": "<sec token>"
}
```

WeChat Work config:
```json
{
  "type": "wechat",
  "access_token": "<token>"
}
```


#### Changelog

* v2.0.0
    * Remove `'a` life cycle
* v1.3.2
    * Add `DingTalk::from_token`
* v1.3.1
    * Add `DingTalk::new_wechat`
* v1.3.0
    * Suports WeChat Work now, add type `"type": "wechat"`, supports method `DingTalk::send_text`
* v1.2.1
    * Remove `maplit` crate
* v1.2.0
    * Use `serde` and `serde_json` crates, replace `json` crate
* v1.1.2
    * Use `hmac` and `sha2` crates, replace `rust-crypto` crate
* v1.1.1
    * `DingTalk::from_json` add `direct_url`
    * Fix problems by clippy
* v1.1.0
    * Change fn to async/.await
* v1.0.1
    * Change two fn names
    * Add readme sample codes
* v1.0.0
    * `TEXT` -> `Text` ..., change enum caps
    * Add `ActionCard` message, send action card message type
    * Add `direct_url` for `DingTalk`, for outgoing robot
    * Implemented almost the functions listed on https://ding-doc.dingtalk.com/doc#/serverapi2/qf2nxq/0fa88adc
* v0.3.0
    * Add `FeedCard` message, send feed card message type
* v0.2.1
    * Add `Dingtalk::from_json`, read token from JSON string
* v0.2.0
    * Add `DingTalk::from_file`, read token from file
* v0.1.2
    * Add `Default::default()` support
* v0.1.1
    * Add `set_default_webhook_url`, default dingtalk webhook url
* v0.1.0
    * Add `DingTalk::send_link(...)`, send link message
* v0.0.3
    * Add `DingTalkMessage` , can set `at_all`, `at_mobiles` now

