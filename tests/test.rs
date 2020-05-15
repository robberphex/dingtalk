use dingtalk::*;

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

    dt.send_message(DingTalkMessage::new_action_card("action card 001", "action card text 001")
        .set_action_card_signle_btn(DingTalkMessageActionCardBtn{
            title: "test signle btn title".into(),
            action_url: "https://hatter.ink/".into(),
        })
    ).await?;

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

    Ok(())
}

