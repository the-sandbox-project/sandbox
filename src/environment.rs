use std::fs;
use std::process::Command;
use std::env;

use crate::editor::get_editor;

pub fn go_to_environment(environment: String) {
    let editor = get_editor(); 

    let beaches_path = "/usr/share/sandbox/beaches/";
    fs::create_dir_all(beaches_path).unwrap();

    env::set_current_dir(beaches_path).unwrap();
    Command::new(editor)
            .arg(".")
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
}
