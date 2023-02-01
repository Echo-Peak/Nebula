use serde_json::{from_str, to_string};
use std::fs;
use std::path::{Path, PathBuf};

mod model;
#[path = "../util.rs"]
mod util;

pub struct ConfigModule {
    config_file: PathBuf,
}

impl ConfigModule {
    pub fn new(&self) -> Result<ConfigModule, std::io::Error> {
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
                Ok(())
            }
            Err(err) => {
                eprintln!("{}", err);
                Err(self.handle_err("Failed to create config file"))
            }
        };

        match util::get_config_file_path() {
            Ok(config_path) => Ok(ConfigModule {
                config_file: config_path,
            }),
            Err(_) => Err(self.handle_err("Unable to instantiate config")),
        }
    }
    fn handle_err(&self, msg: &str) -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::NotFound, msg.to_string())
    }
    pub fn get(&self, sectionID: &str, propID: &str) -> Result<String, std::io::Error> {
        let currentConfig = self.read_config(&self.config_file);
        match currentConfig {
            Ok(current) => match current.read(sectionID, propID) {
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
    pub fn set(&self, sectionID: &str, propID: &str, value: &String) -> Result<(), std::io::Error> {
        let mut currentConfig = self.read_config(&self.config_file);
        match currentConfig {
            Ok(current) => match current.update(sectionID, propID, &value) {
                Ok(updatedConfig) => {
                    self.write_config(&self.config_file, &updatedConfig);
                    Ok(())
                }
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
    fn read_config(&self, config_file: &PathBuf) -> Result<model::ConfigModel, std::io::Error> {
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

    fn write_config(
        &self,
        config_file: &PathBuf,
        updatedConfig: &model::ConfigModel,
    ) -> Result<(), std::io::Error> {
        let configJSON = to_string(&updatedConfig).unwrap();
        match fs::write(config_file.as_path(), configJSON) {
            Ok(_) => Ok(()),
            Err(_) => Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Config file not found",
            )),
        }
    }
}
