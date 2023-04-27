use std::process::exit;
use std::path::Path;

use crate::{get_path, id_is_valid, download::download_environment};

pub async fn install_environment(id: String) {
    if id_is_valid(id.clone()) {
        if in_system(id.clone()) {
            println!("You already have {} installed on your system!", id);
            exit(0)
        } else {
            download_environment(id).await.unwrap()
        }
    } else {
           println!("The environment {} does not exist!", id)
    }
}

pub fn in_system(id: String) -> bool {
    let environment_path = get_path(id.clone());
    let some = environment_path.split("/").collect::<Vec<&str>>()[0].to_owned() + "/" + &id;
    let formatted_path = format!("/usr/share/sandbox/beaches/{}", environment_path);

    let path = Path::new(&formatted_path);

    if path.exists() && path.is_dir() {
        true
    } else {
        false
    }
}
