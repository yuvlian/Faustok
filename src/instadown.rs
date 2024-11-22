use regex::Regex;
use reqwest::Client;
use std::fs::File;
use std::io::Write;
use std::sync::LazyLock;

static REQWEST_CLIENT: LazyLock<Client> =
    LazyLock::new(|| Client::builder().user_agent("TelegramBot").build().unwrap());

static INSTAGRAM_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"https?://(www\.)?(instagram\.com|instagr\.am)/?").unwrap());

pub async fn get_insta(
    r_url: &str,
    filename: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let dd_url = INSTAGRAM_REGEX.replace_all(r_url, "https://d.ddinstagram.com/");
    let response = REQWEST_CLIENT.get(&*dd_url).send().await?;
    let mut file = File::create(filename)?;
    let content = response.bytes().await?;
    file.write_all(&content)?;

    Ok(())
}
