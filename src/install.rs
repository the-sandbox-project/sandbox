use std::process::exit;
use std::path::Path;

use crate::{get_path, id_is_valid, download::download_environment};

pub async fn install_environment(id: String) {
    if id_is_valid(id.clone()) {
        let environment_path = get_path(id.clone()).split("/").collect::<Vec<&str>>()[0].to_owned() + "/" + &id;
        if in_system(environment_path) {
            println!("You already have {} installed on your system!", id);
            exit(0)
        } else {
            download_environment(id).await.unwrap()
        }
    } else {
           println!("The environment {} does not exist!", id)
    }
}

pub fn in_system(path: String) -> bool {
    let formatted_path = format!("/usr/share/sandbox/beaches/{}", path);

    let path = Path::new(&formatted_path);

    if path.exists() && path.is_dir() {
        true
    } else {
        false
    }
}
