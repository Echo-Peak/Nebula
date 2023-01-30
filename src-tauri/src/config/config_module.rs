#[path = "../util.rs"]
mod util;
use model::ConfigModel;

pub struct ConfigModule {
    user_dir: &Path,
}

impl ConfigModule {
    pub fn new() -> Self {
        let user_dir = utils::get_user_directory();
        ConfigModule {
            user_dir: user_dir.as_path(),
        }
    }
    pub fn get(section: &str, prop: &str) -> Option<String, u32, bool> {
        let currentConfig = read_config();
        None
    }
}

fn read_config() -> Result<ConfigModel, Box<dyn std::error::Error>> {
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

fn write_config(config: &ConfigModel) -> Result<(), Box<dyn std::error::Error>> {
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
