# dingtalk

DingTalk util

钉钉机器人 Rust SDK

```rust
pub fn main() {
    let dt = DingTalk::new("<token>", "");
    dt.send_text("Hello world!").ok();
}
```

