use std::path::Path;

use crate::{get_path, id_is_valid, download::download_environment};

pub async fn install_environment(id: String) {
    if id_is_valid(id.clone()).await {
        if in_system(id.clone()).await {
            println!("You already have {} installed on your system!", id);
        } else {
            download_environment(id.clone()).await.unwrap();
        }
    } else {
           println!("The environment {} does not exist!", id)
    }
}

pub async fn in_system(id: String) -> bool {
    let path = get_path(id.clone()).await;
    let environment_path = path.split("/").collect::<Vec<&str>>()[0].to_owned() + "/" + &id;
    let formatted_path = format!("/usr/share/sandbox/beaches/{}", environment_path);

    let path = Path::new(&formatted_path);

    if path.exists() && path.is_dir() {
        true
    } else {
        false
    }
}
