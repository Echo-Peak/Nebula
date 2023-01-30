// extern crate directories;
// use directories::UserDirs;
// use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
// use std::cell::RefCell;
// use std::env;
use std::fs;
// use std::ops::DerefMut;
use std::path::{Path, PathBuf};

mod model;
#[path = "../util.rs"]
mod util;

pub struct ConfigModule {
    config_file: PathBuf,
}

impl ConfigModule {
    pub fn new() -> Option<ConfigModule> {
        let config_file = util::get_config_file_path().unwrap();
        let init_config = match util::init_config_with(&config_file) {
            Ok(wasConfigCreated) => {
                if wasConfigCreated {
                    println!(
                        "The config file was created successfully as {}!",
                        &config_file.display()
                    );
                } else {
                    println!(
                        "The config file {} is already created!",
                        &config_file.display()
                    );
                }
            }
            Err(err) => {
                eprintln!("Failed to create config file: {}", err);
                return None;
            }
        };

        match util::get_config_file_path() {
            Ok(config_path) => Some(ConfigModule {
                config_file: config_path,
            }),
            Err(_) => None,
        }
    }
    pub fn get(&self, sectionID: &str, propID: &str) -> Result<String, std::io::Error> {
        let currentConfig = read_config(&self.config_file);
        match currentConfig {
            Ok(current) => match current.select(sectionID, propID) {
                Ok(val) => Ok(val),
                Err(_) => Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Unable to get value",
                )),
            },
            Err(err) => {
                return Err(err);
            }
        }
    }
    pub fn set(
        &self,
        sectionID: &str,
        propID: &str,
        value: &String,
    ) -> Result<String, std::io::Error> {
        let mut currentConfig = read_config(&self.config_file);
        match currentConfig {
            Ok(current) => match current.update(sectionID, propID) {
                Ok(val) => Ok(val),
                Err(_) => Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Unable to get value",
                )),
            },
            Err(err) => {
                return Err(err);
            }
        }
    }
}

fn read_config(config_file: &PathBuf) -> Result<model::ConfigModel, std::io::Error> {
    match fs::read_to_string(config_file.as_path()) {
        Ok(contents) => {
            let config: model::ConfigModel = from_str(&contents).unwrap();

            Ok(config)
        }
        Err(_) => Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Config file not found",
        )),
    }
}

fn write_config(config_file: &PathBuf) -> Result<model::ConfigModel, std::io::Error> {
    // match fs::read_to_string(config_file.as_path()) {
    //     Ok(contents) => {
    //         let mut config: model::ConfigModel = from_str(&contents).unwrap();

    //         Ok(config)
    //     }
    //     Err(_) => Err(std::io::Error::new(
    //         std::io::ErrorKind::NotFound,
    //         "Config file not found",
    //     )),
    // }
    todo!()
}

// fn check_if_config_file_exists(config_file: &Path) -> bool {
//     match fs::metadata(config_file) {
//         Ok(metadata) => metadata.is_file(),
//         Err(_) => false,
//     }
// }
// fn get_config_location() -> String {
//     if is_production() {
//         if let Some(user_dirs) = UserDirs::new() {
//             let user_dir = user_dirs.home_dir();
//             let nebula_dir = user_dir.join("AppData").join("Roaming").join("Nebula");
//             let nebula_dir_str = nebula_dir.to_str().unwrap();

//             match create_nebula_dir(nebula_dir_str) {
//                 Ok(was_created) => {
//                     if was_created {
//                         println!("Created Nebula directory: {}", nebula_dir_str);
//                     }
//                     return nebula_dir_str.to_string() + "\\config.json";
//                 }
//                 Err(error) => println!("Error creating nebula directory: {}", error),
//             }

//             return "".to_string();
//         }
//     }
//     return "config-dev.json".to_string();
// }

// fn create_initial_config() -> Result<(), Box<dyn std::error::Error>> {
//     let config = Config {
//         credentials: Credentials {
//             access_key_id: "".to_string(),
//             secret_access_key: "".to_string(),
//             region: "".to_string(),
//         },
//         preferences: Preferences {
//             cache: "1GB".to_string(),
//             max_cache_limit: String::from("1GB"),
//             exclusions: String::from(""),
//         },
//         beta_features: BetaFeatures {
//             enable_filesystem_sync: String::from("false"),
//             enable_dark_mode: String::from("false"),
//             enable_viewer: String::from("false"),
//         },
//     };

//     let config_string = serde_json::to_string(&config)?;
//     let location = get_config_location();
//     let path = Path::new(location.as_str());

//     fs::write(path, config_string)?;

//     Ok(())
// }

// pub fn set_config_item(
//     prop_name: &str,
//     prop_value: &str,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     println!("--2");
//     let mut current_config = read_config()?;
//     println!("--3");
//     let mut config_ref_cell = RefCell::new(current_config);
//     let mut config_mut = config_ref_cell.get_mut();
//     let mut config = config_mut.deref_mut();
//     let prop_value_str = prop_value.to_string();

//     println!("--");
//     match prop_name {
//         "credentials.access_key_id" => config.credentials.access_key_id = prop_value_str,
//         "credentials.secret_access_key" => config.credentials.secret_access_key = prop_value_str,
//         "credentials.region" => config.credentials.region = prop_value_str,
//         "preferences.cache" => config.preferences.cache = prop_value_str,
//         "preferences.max_cache_limit" => {
//             config.preferences.max_cache_limit = prop_value_str.parse().unwrap()
//         }
//         "preferences.exclusions" => {
//             config.preferences.exclusions =
//                 prop_value_str.split(',').map(|s| s.to_string()).collect()
//         }
//         "betaFeatures.enableFilesystemSync" => {
//             config.beta_features.enable_filesystem_sync = prop_value_str.to_string();
//         }
//         "betaFeatures.enableDarkMode" => {
//             config.beta_features.enable_dark_mode = prop_value_str.to_string();
//         }
//         "betaFeatures.enableViewer" => {
//             config.beta_features.enable_viewer = prop_value_str.to_string();
//         }
//         _ => println!("Invalid property name"),
//     }
//     match write_config(&config) {
//         Ok(()) => {
//             println!("Config file was updated successfully");
//             Ok(())
//         }
//         Err(error) => {
//             println!("Error writing config file");
//             Err(error)
//         }
//     }
// }

// pub fn get_config_item(prop_name: &str) -> Result<String, Box<dyn std::error::Error>> {
//     let current_config = read_config()?;
//     let mut prop_value = "".to_string();

//     match prop_name {
//         "credentials.access_key_id" => prop_value = current_config.credentials.access_key_id,
//         "credentials.secret_access_key" => {
//             prop_value = current_config.credentials.secret_access_key
//         }
//         "credentials.region" => prop_value = current_config.credentials.region,
//         "preferences.cache" => prop_value = current_config.preferences.cache,
//         "preferences.max_cache_limit" => prop_value = current_config.preferences.max_cache_limit,
//         "preferences.exclusions" => {
//             prop_value = current_config.preferences.exclusions;
//         }
//         "betaFeatures.enableFilesystemSync" => {
//             prop_value = current_config.beta_features.enable_filesystem_sync;
//         }
//         "betaFeatures.enableDarkMode" => {
//             prop_value = current_config.beta_features.enable_dark_mode;
//         }
//         "betaFeatures.enableViewer" => {
//             prop_value = current_config.beta_features.enable_viewer;
//         }
//         _ => println!("Invalid property name"),
//     }
//     Ok(prop_value)
// }

// pub fn handle_get_config_item(result: Result<String, Box<dyn std::error::Error>>) -> String {
//     match result {
//         Ok(prop_value) => prop_value.to_string(),
//         Err(error) => format!("Error: {}", error),
//     }
// }

// pub fn create_config() {
//     let config_file_location = get_config_location();
//     let config_path = Path::new(config_file_location.as_str());
//     let exists = check_if_config_file_exists(config_path);

//     if !exists {
//         println!("Config file does not exist yet. Creating it now");
//         match create_initial_config() {
//             Ok(()) => println!("Config file created at {}", config_file_location),
//             Err(error) => println!("Error creating config file: {}", error),
//         }
//     } else {
//         println!("The config file \"{}\" already exist", config_file_location);
//     }
// }
