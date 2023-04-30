use std::process::Command;
use std::env;

use crate::editor::get_editor;
use crate::get_path;

pub async fn open_environment(environment: String) {
    let editor = get_editor(); 

    let path = get_path(environment.clone()).await;
    let environment_path = path.split("/").collect::<Vec<&str>>()[0].to_owned() + "/" + &environment;

    let beaches_path = match env::consts::OS {
        "windows" => {
            let appdata = std::env::var("appdata").unwrap();
            let beaches_path = format!("{}/sandbox/beaches/{}", appdata, environment_path);
            beaches_path
        }
        _ => {
            let beaches_path = format!("/usr/share/sandbox/beaches/{}", environment_path);
            beaches_path
        }
    };

    env::set_current_dir(beaches_path).unwrap();
    Command::new(editor)
            .arg(".")
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
}

