use std::process::Command;
use std::process::exit;

use crate::config::read_config_file;

pub fn get_editor() -> String {
    let config = read_config_file().unwrap();
    let editor = config.editor.editor_name;

    match Command::new(&editor).status() {
        Ok(_) => editor,
        Err(_) => {
            println!("[ERROR] EDITOR_NOT_FOUND: There was an Error Running the Editor: {}. Check your sandbox.yml Config.", editor);
            exit(1);
        }
    }
}
