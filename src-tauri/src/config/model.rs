use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ConfigModel {
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

impl ConfigModel {
    fn handle_missing_prop(&self, sectionID: &str, propID: &str) -> std::io::Error {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!(
                "The prop '{}' is not assignable for section '{}'",
                propID, sectionID
            ),
        )
    }
    pub fn read(&self, sectionID: &str, propID: &str) -> Result<String, std::io::Error> {
        match sectionID {
            "credentials" => match propID {
                "access_key_id" => Ok(self.credentials.access_key_id),
                "secret_access_key" => Ok(self.credentials.secret_access_key),
                "region" => Ok(self.credentials.region),
            },
            "preferences" => match propID {
                "cache" => Ok(self.preferences.cache),
                "max_cache_limit" => Ok(self.preferences.max_cache_limit),
                "exclusions" => Ok(self.preferences.exclusions),
            },
            "beta_features" => match propID {
                "enable_filesystem_sync" => Ok(self.beta_features.enable_filesystem_sync),
                "enable_dark_mode" => Ok(self.beta_features.enable_dark_mode),
                "enable_viewer" => Ok(self.beta_features.enable_viewer),
            },
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Config file not found",
            )),
        }
    }
    pub fn update(
        &self,
        sectionID: &str,
        propID: &str,
        value: &String,
    ) -> Result<&ConfigModel, std::io::Error> {
        let mut newInstance = self.clone();
        match sectionID {
            "credentials" => match propID {
                "access_key_id" => newInstance.credentials.access_key_id = value.to_string(),
                "secret_access_key" => {
                    newInstance.credentials.secret_access_key = value.to_string()
                }
                "region" => newInstance.credentials.region = value.to_string(),
                _ => return Err(self.handle_missing_prop(sectionID, propID)),
            },
            "preferences" => match propID {
                "cache" => newInstance.preferences.cache = value.to_string(),
                "max_cache_limit" => newInstance.preferences.max_cache_limit = value.to_string(),
                "exclusions" => newInstance.preferences.exclusions = value.to_string(),
                _ => return Err(self.handle_missing_prop(sectionID, propID)),
            },
            "beta_features" => match propID {
                "enable_filesystem_sync" => {
                    newInstance.beta_features.enable_filesystem_sync = value.to_string()
                }
                "enable_dark_mode" => {
                    newInstance.beta_features.enable_dark_mode = value.to_string()
                }
                "enable_viewer" => newInstance.beta_features.enable_viewer = value.to_string(),
                _ => return Err(self.handle_missing_prop(sectionID, propID)),
            },
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("There is no such property \'{}\'", propID),
                ))
            }
        }
        Ok(&newInstance)
    }
}

fn build_config() -> ConfigModel {
    let config = ConfigModel {
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
    return config;
}

pub fn create_as_json() -> Result<String, serde_json::Error> {
    let config = build_config();
    match serde_json::to_string(&config) {
        Ok(json) => Ok(json),
        Err(e) => Err(e),
    }
}
