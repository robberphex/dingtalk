# dingtalk

DingTalk Robot Util, Send text/markdown/link messages using DingTalk robot

钉钉机器人 Rust SDK


Sample 1:
```rust
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dt = DingTalk::new("<token>", "");
    dt.send_text("Hello world!")?;

    Ok(())
}
```

Sample 2 (Read token from file):
```rust
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dt = DingTalk::from_file("~/.dingtalk-token.json")?;
    dt.send_text("Hello world!")?;

    Ok(())
}
```


#### Changelog

* v0.4.0
    * `TEXT` -> `Text` ..., change enum caps
    * Add `ActionCard` message, send action card message type
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

