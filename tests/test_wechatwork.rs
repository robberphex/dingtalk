use dingtalk::*;

#[test]
fn run_all_tests_wechat_work() {
    tokio_test::block_on(_test_send_wechat_work()).unwrap();
}

async fn _test_send_wechat_work() -> XResult<()> {
    let dt = DingTalk::from_file("~/.wechat-work-token.json")?;
    dt.send_text("test message 001 ---------------------").await?;
    Ok(())
}

