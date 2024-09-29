use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
pub struct Token {
    pub token: String,
}

pub fn get_token() -> Result<Token, Box<dyn std::error::Error>> {
    // Check if the token file exists
    if !Path::new("Token.json").exists() {
        return Err("Token.json file does not exist".into());
    }

    // Read the token file
    let token_file = fs::read_to_string("Token.json")
        .map_err(|e| format!("Failed to read Token.json: {}", e))?;

    // Parse the token from JSON
    let token: Token = serde_json::from_str(&token_file)
        .map_err(|e| format!("Failed to parse Token.json: {}", e))?;

    Ok(token)
}

#[derive(Serialize, Deserialize)]
pub struct UserSetting {
    pub user_map: HashMap<String, bool>,
}

pub fn get_user_setting() -> Result<UserSetting, Box<dyn std::error::Error + Send + Sync>> {
    // Check if the user settings file exists
    if !Path::new("User.json").exists() {
        return Err("User.json file does not exist".into());
    }

    // Read the user settings file
    let user_file =
        fs::read_to_string("User.json").map_err(|e| format!("Failed to read User.json: {}", e))?;

    // Parse the user settings from JSON
    let user_setting: UserSetting = serde_json::from_str(&user_file)
        .map_err(|e| format!("Failed to parse User.json: {}", e))?;

    Ok(user_setting)
}

pub fn update_user_setting(
    user_id: &str,
    value: bool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Read the existing user settings file
    let user_file =
        fs::read_to_string("User.json").map_err(|e| format!("Failed to read User.json: {}", e))?;

    // Parse the existing user settings from JSON
    let mut user_setting: UserSetting = serde_json::from_str(&user_file)
        .map_err(|e| format!("Failed to parse User.json: {}", e))?;

    // Update the user map with the new value
    user_setting.user_map.insert(user_id.to_string(), value);

    // Serialize the updated user settings back to JSON
    let updated_content = serde_json::to_string_pretty(&user_setting)
        .map_err(|e| format!("Failed to serialize updated user settings: {}", e))?;

    // Write the updated content back to the user settings file
    fs::write("User.json", updated_content)
        .map_err(|e| format!("Failed to write User.json: {}", e))?;

    Ok(())
}
