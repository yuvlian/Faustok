use reqwest::Client;
use serde::Deserialize;
use std::fs::File;
use std::io::Write;
use std::sync::LazyLock;

static REQWEST_CLIENT: LazyLock<Client> = LazyLock::new(Client::new);

static TIKLYDOWN_BASE_URL: LazyLock<String> =
    LazyLock::new(|| String::from("https://api.tiklydown.eu.org/api/download?url="));

#[derive(Deserialize)]
pub struct TiklydownApiOne {
    pub images: Option<Vec<ImageStuff>>,
    pub video: Option<VideoStuff>,
    pub music: Option<MusicStuff>,
}

#[derive(Deserialize)]
pub struct VideoStuff {
    #[serde(rename = "noWatermark")]
    pub video_url: String,
}

#[derive(Deserialize)]
pub struct ImageStuff {
    #[serde(rename = "url")]
    pub image_url: String,
}

#[derive(Deserialize)]
pub struct MusicStuff {
    #[serde(rename = "play_url")]
    pub music_url: String,
}

fn format_url(url: &str) -> String {
    format!("{}{}", *TIKLYDOWN_BASE_URL, url)
}

pub async fn get_video(
    v_url: &str,
    filename: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let url = format_url(v_url);
    let response = REQWEST_CLIENT.get(&url).send().await?;
    let api_response: TiklydownApiOne = response.json().await?;

    if let Some(video) = api_response.video {
        let video_response = REQWEST_CLIENT.get(&video.video_url).send().await?;
        let mut file = File::create(filename)?;
        let content = video_response.bytes().await?;
        file.write_all(&content)?;
    } else {
        return Err("Video URL not found in the response".into());
    }

    Ok(())
}

pub async fn get_image(
    i_url: &str,
    userid: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    let url = format_url(i_url);
    let response = REQWEST_CLIENT.get(&url).send().await?;
    let api_response: TiklydownApiOne = response.json().await?;

    let mut filenames = Vec::new();

    if let Some(images) = api_response.images {
        for (index, image) in images.iter().enumerate() {
            let image_response = REQWEST_CLIENT.get(&image.image_url).send().await?;
            let filename = format!("{}_{}.jpg", userid, index);
            let mut file = File::create(&filename)?;
            let content = image_response.bytes().await?;
            file.write_all(&content)?;
            filenames.push(filename);
        }
    } else {
        return Err("Images not found in the response".into());
    }

    Ok(filenames)
}

pub async fn get_music(
    m_url: &str,
    filename: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let url = format_url(m_url);
    let response = REQWEST_CLIENT.get(&url).send().await?;
    let api_response: TiklydownApiOne = response.json().await?;

    if let Some(music) = api_response.music {
        let music_response = REQWEST_CLIENT.get(&music.music_url).send().await?;
        let mut file = File::create(filename)?;
        let content = music_response.bytes().await?;
        file.write_all(&content)?;
    } else {
        return Err("Music URL not found in the response".into());
    }

    Ok(())
}
