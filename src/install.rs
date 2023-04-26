use serde_yaml::Value;
use std::fs;

use crate::{get_path, id_is_valid, download::download_environment};

pub async fn install_environment(id: String) {
    if id_is_valid(id.clone()) {
        let environment_path = get_path(id.clone());
        if in_system(environment_path) {
            println!("You already have {} installed on your system!", id)
        } else {
            download_environment(id).await.unwrap()
        }
    } else {
           println!("The environment {} does not exist!", id)
    }
}

pub fn in_system(path: String) -> bool {
    let formatted_path = format!("/usr/share/sandbox/beaches/{}", path);

    let installed = match fs::read(formatted_path) {
        Ok(_) => true,
        Err(_) => false
    };

    installed
}
