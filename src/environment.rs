use std::fs;
use std::process::Command;
use std::env;

use crate::editor::get_editor;

pub fn open_environment(environment: String) {
    let editor = get_editor(); 

    let beaches_path = format!("/usr/share/sandbox/beaches/{}", environment);

    env::set_current_dir(beaches_path).unwrap();
    Command::new(editor)
            .arg(".")
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
}

pub fn setup_environment(environment: String) {
    let beaches_path = format!("/usr/share/sandbox/beaches/{}", environment);

    let environment_exists = match fs::metadata(beaches_path) {
        Ok(_) => true,
        Err(_) => false,
    };

    if environment_exists {
        open_environment(environment)
    } else {
    }
}
