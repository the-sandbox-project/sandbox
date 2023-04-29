use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::env;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub editor: EditorConfig,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EditorConfig {
    pub editor_name: String,
}

pub fn read_config_file() -> Result<Config, String> { 
    match env::consts::OS {
        "linux" | "darwin" => {
            let config = get_config_linux().unwrap();
            Ok(config)
        }, 
        "windows" => { 
            let not_supported = String::from("u needa do this");
            Err(not_supported)
        }
        _ => {
            let not_supported = String::from("This OS is Not Supported!");
            Err(not_supported)
        }
    }
}

fn get_config_linux() -> Result<Config, String>{
    let home_dir = env::var("HOME").unwrap();
    let config_dir = format!("{}/.config/sandbox/", home_dir);
    env::set_current_dir(config_dir).unwrap();
    
    let mut file = File::open("sandbox.yml").map_err(|_| "We cannot read the sandbox.yml! Does it exist?".to_string())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| e.to_string())?;

    let config: Config = serde_yaml::from_str(&contents).map_err(|e| e.to_string())?;
    Ok(config)
}

fn get_config_windows() {
    todo!("windows")
}

