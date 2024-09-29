mod database;
mod tiklydown;
use database::*;
use poise::serenity_prelude as serenity;
use regex::Regex;
use std::fs;
use std::sync::LazyLock;
use tiklydown::{get_image, get_music, get_video};
use tokio::sync::RwLock;

include!("misc.rs"); // Contains constants: SLIDESHOW_CHUNK_VALUE, BOT_PREFIX, and HELP_COMMAND

// Define a regex pattern to match TikTok URLs.
static TIKTOK_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"https?://(www\.)?(tiktok\.com|vt\.tiktok\.com|vm\.tiktok\.com)/?").unwrap()
});

static TNKTOK_URL: LazyLock<String> = LazyLock::new(|| String::from("https://tnktok.com/"));

// Struct to hold shared data across commands.
struct Data {
    user_settings: RwLock<UserSetting>,
}

// Define type for error handling.
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Command to display bot information.
#[poise::command(slash_command, prefix_command)]
async fn about(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say(ABOUT_COMMAND).await?;
    Ok(())
}

/// Command to display help information.
#[poise::command(slash_command, prefix_command)]
async fn help(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say(HELP_COMMAND).await?;
    Ok(())
}

/// Command to toggle the auto-fix feature for link replacement.
#[poise::command(slash_command, prefix_command)]
async fn autofix(
    ctx: Context<'_>,
    #[description = "Value to set (true or false)"] value: bool,
) -> Result<(), Error> {
    let user_id = ctx.author().id.to_string();

    // Update user settings with the new value.
    {
        let mut settings = ctx.data().user_settings.write().await;
        settings.user_map.insert(user_id.clone(), value);
    }
    update_user_setting(&user_id, value)?;

    ctx.say(format!(
        r#"Autofix for id "{}" has been updated to **{}**"#,
        user_id, value
    ))
    .await?;
    Ok(())
}

/// Command to check the current state of the auto-fix toggle.
#[poise::command(slash_command, prefix_command)]
async fn check_autofix(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let user_id = user
        .as_ref()
        .map_or_else(|| ctx.author().id.to_string(), |u| u.id.to_string());
    let settings = ctx.data().user_settings.read().await;
    let value = settings.user_map.get(&user_id).unwrap_or(&false);

    ctx.say(format!(
        r#"Autofix for id "{}" is currently **{}**"#,
        user_id, value
    ))
    .await?;
    Ok(())
}

/// Command to download a TikTok video.
#[poise::command(slash_command, prefix_command)]
async fn vid(ctx: Context<'_>, #[description = "TikTok URL"] url: String) -> Result<(), Error> {
    // So that the interaction doesn't expire, we defer it.
    ctx.defer().await?;

    let user_id = ctx.author().id.to_string();
    let video_filename = format!("{}.mp4", user_id); // Video file named after user ID
    get_video(&url, &video_filename).await?;

    // Create and send the video attachment.
    let attachment =
        serenity::CreateAttachment::path(std::path::Path::new(&video_filename)).await?;
    ctx.send(poise::reply::CreateReply {
        attachments: vec![attachment],
        ..Default::default()
    })
    .await?;

    // Remove the file after sending to free up resources.
    fs::remove_file(&video_filename)?;
    Ok(())
}

/// Command to download images from a TikTok slideshow.
#[poise::command(slash_command, prefix_command)]
async fn img(ctx: Context<'_>, #[description = "TikTok URL"] url: String) -> Result<(), Error> {
    // So that the interaction doesn't expire, we defer it.
    ctx.defer().await?;

    let user_id = ctx.author().id.to_string();
    let image_filenames = get_image(&url, &user_id).await?; // Get images as a vector of filenames

    // Create attachments for each image file.
    let mut attachments = Vec::with_capacity(image_filenames.len());
    for filename in &image_filenames {
        let attachment = serenity::CreateAttachment::path(std::path::Path::new(filename)).await?;
        attachments.push(attachment);
    }

    // Send attachments in chunks based on the slideshow configuration.
    for chunk in attachments.chunks(SLIDESHOW_CHUNK_VALUE) {
        let reply_attachment = poise::reply::CreateReply {
            attachments: chunk.to_vec(),
            ..Default::default()
        };
        ctx.send(reply_attachment).await?;
    }

    // Clean up by removing the image files after sending.
    for filename in image_filenames {
        fs::remove_file(&filename)?;
    }
    Ok(())
}

/// Command to download music from a TikTok video.
#[poise::command(slash_command, prefix_command)]
async fn mp3(ctx: Context<'_>, #[description = "TikTok URL"] url: String) -> Result<(), Error> {
    // So that the interaction doesn't expire, we defer it.
    ctx.defer().await?;

    let user_id = ctx.author().id.to_string();
    let music_filename = format!("{}.mp3", user_id); // Music file named after user ID
    get_music(&url, &music_filename).await?;

    // Create and send the music attachment.
    let attachment =
        serenity::CreateAttachment::path(std::path::Path::new(&music_filename)).await?;
    ctx.send(poise::reply::CreateReply {
        attachments: vec![attachment],
        ..Default::default()
    })
    .await?;

    // Clean up by removing the music file after sending.
    fs::remove_file(&music_filename)?;
    Ok(())
}

#[tokio::main]
async fn main() {
    // Retrieve the bot token from configuration.
    let token = get_token().expect("Failed to get token").token;

    // Define the necessary gateway intents for the bot.
    let intents = serenity::GatewayIntents::GUILD_MESSAGES
        | serenity::GatewayIntents::DIRECT_MESSAGES
        | serenity::GatewayIntents::MESSAGE_CONTENT;

    // Load user settings.
    let user_settings = get_user_setting().expect("Failed to get user settings");
    let data = Data {
        user_settings: RwLock::new(user_settings),
    };

    let commands_list = vec![
        autofix(),
        check_autofix(),
        vid(),
        img(),
        mp3(),
        help(),
        about(),
    ];

    // Build the command framework.
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands_list,
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(BOT_PREFIX.into()),
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(data)
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}

/// Event handler for various bot events.
async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            println!("Logged in as {}", data_about_bot.user.name);
        }
        serenity::FullEvent::Message { new_message } => {
            // Get autofix settings
            let settings = data.user_settings.read().await;
            let user_id = new_message.author.id.to_string();

            // Get message string
            let content = &new_message.content;

            // Get bools
            let should_replace = *settings.user_map.get(&user_id).unwrap_or(&false);
            let is_command =
                content.contains(".vid") || content.contains(".img") || content.contains(".mp3");
            let is_tiktok = TIKTOK_REGEX.is_match(content);

            // Bool check
            match (is_command, should_replace, is_tiktok) {
                (true, _, true) => {
                    // If a command is present and has a tiktok url, suppress embeds.
                    let remove_embed = serenity::MessageFlags::SUPPRESS_EMBEDS;
                    let embed_remover = serenity::EditMessage::new().flags(remove_embed);
                    new_message.clone().edit(ctx, embed_remover).await?;
                }
                (false, true, true) => {
                    // If auto-replace is enabled and the message contains a TikTok link, replace the link and suppress embeds.
                    let tiktok_reply = TIKTOK_REGEX.replace_all(content, &*TNKTOK_URL);
                    let remove_embed = serenity::MessageFlags::SUPPRESS_EMBEDS;
                    let embed_remover = serenity::EditMessage::new().flags(remove_embed);

                    new_message.clone().edit(ctx, embed_remover).await?;
                    new_message.reply(ctx, tiktok_reply).await?;
                }
                _ => {}
            }
        }
        _ => {}
    }
    Ok(())
}
