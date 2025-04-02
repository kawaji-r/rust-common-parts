use fantoccini::{Client, ClientBuilder, Locator};
use std::process::{Child, Command, Stdio};
use std::time::Duration;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // geckodriverを指定のポート(4444)で起動
    let mut geckodriver: Child = Command::new("geckodriver")
        .arg("--port")
        .arg("4444")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

    // geckodriverの起動待ち(2秒程度)
    tokio::time::sleep(Duration::from_secs(2)).await;

    // ClientBuilder::native() を使用してWebDriverサーバー(geckodriver)に接続
    let mut client = ClientBuilder::native()
        .connect("http://localhost:4444")
        .await?;

    // 指定したURLへ移動
    client.goto("https://www.example.com").await?;

    // CSSセレクタを使用して最初のリンクをクリックする例
    let element = client.find(Locator::Css("a")).await?;
    element.click().await?;

    // 操作終了後にクライアントを閉じる
    client.close().await?;

    // geckodriverプロセスを終了
    geckodriver.kill()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use fantoccini::{Client, ClientBuilder, Locator};
    use std::process::{Child, Command, Stdio};
    use std::time::Duration;
    use tokio;

    #[tokio::test]
    async fn test_firefox_navigation() -> Result<(), Box<dyn std::error::Error>> {
        // geckodriverを指定のポート(4444)で起動
        let mut geckodriver: Child = Command::new("geckodriver")
            .arg("--port")
            .arg("4444")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;

        // geckodriverが起動するまで少し待機
        tokio::time::sleep(Duration::from_secs(2)).await;

        // ClientBuilder::native() を使用してWebDriverサーバーに接続
        let mut client = ClientBuilder::native()
            .connect("http://localhost:4444")
            .await?;

        // 指定したURLへ移動
        client.goto("https://www.example.com").await?;

        // ページタイトルを取得して検証
        let title = client.title().await?;
        assert_eq!(title, "Example Domain");

        // 操作終了後にクライアントを閉じる
        client.close().await?;

        // geckodriverプロセスを終了
        geckodriver.kill()?;
        Ok(())
    }
}
