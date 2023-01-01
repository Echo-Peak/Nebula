extern crate directories;
use directories::UserDirs;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
use std::cell::RefCell;
use std::env;
use std::fs;
use std::ops::DerefMut;
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct Config {
    credentials: Credentials,
    preferences: Preferences,
    beta_features: BetaFeatures,
}

#[derive(Serialize, Deserialize)]
struct Credentials {
    access_key_id: String,
    secret_access_key: String,
    region: String,
}

#[derive(Serialize, Deserialize)]
struct Preferences {
    cache: String,
    max_cache_limit: String,
    exclusions: String,
}
#[derive(Serialize, Deserialize)]
struct BetaFeatures {
    enable_filesystem_sync: String,
    enable_dark_mode: String,
    enable_viewer: String,
}

fn is_production() -> bool {
    match env::var_os("NODE_ENV") {
        Some(val) => val == "production",
        None => false,
    }
}

fn create_nebula_dir(dir_path: &str) -> Result<bool, std::io::Error> {
    match fs::metadata(dir_path) {
        Ok(metadata) => {
            if metadata.is_dir() {
                Ok(false)
            } else {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Path exists but is not a directory",
                ))
            }
        }
        Err(error) => {
            if error.kind() == std::io::ErrorKind::NotFound {
                fs::create_dir(dir_path)?;
                Ok(true)
            } else {
                Err(error)
            }
        }
    }
}

fn read_config() -> Result<Config, Box<dyn std::error::Error>> {
    let location = get_config_location();
    let path = Path::new(location.as_str());
    let exists = check_if_config_file_exists(path);
    if exists {
        let contents = fs::read_to_string(path)?;
        let config: Config = from_str(&contents).unwrap();
        Ok(config)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Config file not found",
        )))
    }
}

fn write_config(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let location = get_config_location();
    let path = Path::new(location.as_str());
    let exists = check_if_config_file_exists(path);

    if exists {
        let contents = to_string(config)?;
        fs::write(path, contents)?;
        Ok(())
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Config file not found",
        )))
    }
}

fn check_if_config_file_exists(config_file: &Path) -> bool {
    match fs::metadata(config_file) {
        Ok(metadata) => metadata.is_file(),
        Err(_) => false,
    }
}
fn get_config_location() -> String {
    if is_production() {
        if let Some(user_dirs) = UserDirs::new() {
            let user_dir = user_dirs.home_dir();
            let nebula_dir = user_dir.join("AppData").join("Roaming").join("Nebula");
            let nebula_dir_str = nebula_dir.to_str().unwrap();

            match create_nebula_dir(nebula_dir_str) {
                Ok(was_created) => {
                    if was_created {
                        println!("Created Nebula directory: {}", nebula_dir_str);
                    }
                    return nebula_dir_str.to_string() + "\\config.json";
                }
                Err(error) => println!("Error creating nebula directory: {}", error),
            }

            return "".to_string();
        }
    }
    return "config-dev.json".to_string();
}

fn create_initial_config() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config {
        credentials: Credentials {
            access_key_id: "".to_string(),
            secret_access_key: "".to_string(),
            region: "".to_string(),
        },
        preferences: Preferences {
            cache: "1GB".to_string(),
            max_cache_limit: String::from("1GB"),
            exclusions: String::from(""),
        },
        beta_features: BetaFeatures {
            enable_filesystem_sync: String::from("false"),
            enable_dark_mode: String::from("false"),
            enable_viewer: String::from("false"),
        },
    };

    let config_string = serde_json::to_string(&config)?;
    let location = get_config_location();
    let path = Path::new(location.as_str());

    fs::write(path, config_string)?;

    Ok(())
}

pub fn set_config_item(
    prop_name: &str,
    prop_value: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("--2");
    let mut current_config = read_config()?;
    println!("--3");
    let mut config_ref_cell = RefCell::new(current_config);
    let mut config_mut = config_ref_cell.get_mut();
    let mut config = config_mut.deref_mut();
    let prop_value_str = prop_value.to_string();

    println!("--");
    match prop_name {
        "credentials.access_key_id" => config.credentials.access_key_id = prop_value_str,
        "credentials.secret_access_key" => config.credentials.secret_access_key = prop_value_str,
        "credentials.region" => config.credentials.region = prop_value_str,
        "preferences.cache" => config.preferences.cache = prop_value_str,
        "preferences.max_cache_limit" => {
            config.preferences.max_cache_limit = prop_value_str.parse().unwrap()
        }
        "preferences.exclusions" => {
            config.preferences.exclusions =
                prop_value_str.split(',').map(|s| s.to_string()).collect()
        }
        "betaFeatures.enableFilesystemSync" => {
            config.beta_features.enable_filesystem_sync = prop_value_str.to_string();
        }
        "betaFeatures.enableDarkMode" => {
            config.beta_features.enable_dark_mode = prop_value_str.to_string();
        }
        "betaFeatures.enableViewer" => {
            config.beta_features.enable_viewer = prop_value_str.to_string();
        }
        _ => println!("Invalid property name"),
    }
    match write_config(&config) {
        Ok(()) => {
            println!("Config file was updated successfully");
            Ok(())
        }
        Err(error) => {
            println!("Error writing config file");
            Err(error)
        }
    }
}

pub fn get_config_item(prop_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let current_config = read_config()?;
    let mut prop_value = "".to_string();

    match prop_name {
        "credentials.access_key_id" => prop_value = current_config.credentials.access_key_id,
        "credentials.secret_access_key" => {
            prop_value = current_config.credentials.secret_access_key
        }
        "credentials.region" => prop_value = current_config.credentials.region,
        "preferences.cache" => prop_value = current_config.preferences.cache,
        "preferences.max_cache_limit" => prop_value = current_config.preferences.max_cache_limit,
        "preferences.exclusions" => {
            prop_value = current_config.preferences.exclusions;
        }
        "betaFeatures.enableFilesystemSync" => {
            prop_value = current_config.beta_features.enable_filesystem_sync;
        }
        "betaFeatures.enableDarkMode" => {
            prop_value = current_config.beta_features.enable_dark_mode;
        }
        "betaFeatures.enableViewer" => {
            prop_value = current_config.beta_features.enable_viewer;
        }
        _ => println!("Invalid property name"),
    }
    Ok(prop_value)
}

pub fn handle_get_config_item(result: Result<String, Box<dyn std::error::Error>>) -> String {
    match result {
        Ok(prop_value) => prop_value.to_string(),
        Err(error) => format!("Error: {}", error),
    }
}

pub fn create_config() {
    let config_file_location = get_config_location();
    let config_path = Path::new(config_file_location.as_str());
    let exists = check_if_config_file_exists(config_path);

    if !exists {
        println!("Config file does not exist yet. Creating it now");
        match create_initial_config() {
            Ok(()) => println!("Config file created at {}", config_file_location),
            Err(error) => println!("Error creating config file: {}", error),
        }
    } else {
        println!("The config file \"{}\" already exist", config_file_location);
    }
}
