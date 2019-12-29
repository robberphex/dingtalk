# dingtalk

DingTalk util

钉钉机器人 Rust SDK

```rust
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dt = DingTalk::new("<token>", "");
    dt.send_text("Hello world!")?;

    Ok(())
}
```


#### Changelog

* v0.2.0
    * Add `DingTalk::from_file`, read token from file
* v0.1.2
    * Add `Default::default()` support
* v0.1.1
    * Add `set_default_webhook_url`, default dingtalk webhook url
* v0.1.0
    * Add `DingTalk::send_link(...)`, send link message
* v0.0.3
    * Add `DingTalkMessage` , can set at_all, at_mobiles now

