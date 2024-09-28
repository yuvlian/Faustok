// Be sure to update HELP_COMMAND if changed
const BOT_PREFIX: &str = ".";

// For how many images per message
const SLIDESHOW_CHUNK_VALUE: usize = 4;

const ABOUT_COMMAND: &str = 
r###"[about]
dev = ["yuvlian"]
repo = "<https://github.com/yuvlian/Faustok>"
license = "BSD 3-Clause"

[version]
bot = "0.1.0"
poise = "0.6.1"
rust = "1.83.0-nightly"
"###;

const HELP_COMMAND: &str = 
r###"[commands]
help = "Shows this message"
about = "Shows information about the bot"
autofix = "Enables autoreplace links in messages with the embed fixed"
check_autofix = "Check autofix status"
vid = "Downloads video from a tiktok video url and reuploads it"
mp3 = "Downloads audio from a tiktok video url and reuploads it"
img = "Downloads images from a tiktok slideshow url and reuploads it"

[commands.example]
help = ".help"
about = ".about"
autofix = ".autofix true"
check_autofix = ".check_autofix @yuvlian"
vid = ".vid <https://vt.tiktok.com/ZS2fMDLkU/>"
mp3 = ".mp3 <https://vt.tiktok.com/ZS2fMDLkU/>"
img = ".img <https://vt.tiktok.com/ZS2g5AWKk/>"
"###;