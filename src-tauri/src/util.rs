use std::env;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

#[path = "./config/model.rs"]
mod model;

pub fn create_config_file(config_file: &PathBuf) -> Result<bool, std::io::Error> {
    return match fs::create_dir_all(config_file.as_path()) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    };
}
fn split_path_filename(path: &Path) -> (Option<&str>, Option<&str>) {
    let file_name = path.file_name();
    let file_name = file_name.and_then(|f| f.to_str());
    let dir = path.parent();
    let dir = dir.and_then(|p| p.to_str());
    (file_name, dir)
}

pub fn is_production() -> bool {
    match env::var_os("NODE_ENV") {
        Some(val) => val == "production",
        None => false,
    }
}

fn get_user_directory() -> PathBuf {
    let home_dir = env::var("HOME").unwrap().to_string();
    let path = PathBuf::from(&home_dir);
    let nebula_dir = path.join("AppData").join("Roaming").join("Nebula");
    nebula_dir
}

fn does_nebula_directory_exists() -> bool {
    let nebPath = get_user_directory();
    let path = Path::new(nebPath.as_path());
    let exists = path.exists();
    if exists {
        return true;
    }
    false
}

fn get_cwd() -> Option<PathBuf> {
    let cwd = env::current_dir();
    match cwd {
        Ok(path) => Some(path),
        Err(e) => None,
    }
}

pub fn get_config_file_path() -> Result<PathBuf, std::io::Error> {
    let is_prod = is_production();
    if !is_prod {
        match get_cwd() {
            Some(cwd) => Ok(cwd.join("config.json")),
            None => return Err(Error::new(ErrorKind::NotFound, "Unable to get CWD")),
        }
    } else {
        let nebPath = get_user_directory();
        Ok(nebPath.join("config.json"))
    }
}

fn create_file(fpath: &PathBuf, contents: String) -> Result<(), std::io::Error> {
    match fs::write(fpath.as_path(), contents) {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::new(ErrorKind::Other, e.to_string())),
    }
}
pub fn init_config_with(config_file: &PathBuf) -> Result<bool, std::io::Error> {
    let path = Path::new(&config_file.as_path());
    if path.exists() {
        Ok(false)
    } else {
        let config_json = model::create_as_json()?;
        match create_file(&config_file, config_json) {
            Ok(_) => Ok(true),
            Err(e) => Err(Error::new(e.kind(), e.to_string())),
        }
    }
}
