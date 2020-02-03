use regex::Regex;
use std::fs::File;
use std::io::Write as _;
use anyhow::Result;
use futures::future::join_all;

async fn download_game_ai_pro(version: u32) -> Result<()> {
    let base = if version > 1 { format!("http://www.gameaipro.com/GameAIPro{}", version) } else { "http://www.gameaipro.com/GameAIPro".to_string() };
    let text = reqwest::get(&base)
        .await?
        .text()
        .await?;

    let re = Regex::new(r#""GameAIPro(.*)\.pdf""#)?;
    for v in re.captures_iter(&text) {
        let filename = &v[0].trim_matches('"');
        let filepath = format!("download/{}", filename);
        let url = format!("{}/{}", base, filename);
        let mut file = File::create(filepath)?;
        let mut bytes = reqwest::get(&url).await?.bytes().await?;
        file.write_all(&mut bytes)?;
        println!("downloaded: {}", filename);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let download_dir = std::path::Path::new("download");
    if !download_dir.exists() {
        std::fs::create_dir(download_dir)?;
    }
    let tasks = vec![ download_game_ai_pro(1), download_game_ai_pro(2), download_game_ai_pro(3)];
    let _ = join_all(tasks).await;
    println!("finish");
    Ok(())
}
