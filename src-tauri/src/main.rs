#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod config;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_config_item(prop_name: &str) -> String {
    let config_item = config::get_config_item(prop_name);
    let result = config::handle_get_config_item(config_item);
    return result;
}

#[tauri::command]
fn set_config_item(prop_name: &str, prop_value: &str) -> Result<(), Box<dyn std::error::Error>> {
    match config::set_config_item(prop_name, prop_value) {
        Ok(_) => {
            println!(
                "Successfully set property \"{}\" to \"{}\"",
                prop_name, prop_value
            );
            Ok(())
        }
        Err(err) => {
            println!("Failed to set the property \"{}\"", prop_name);
            Err(err)
        }
    }
}

fn main() {
    config::create_config();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
