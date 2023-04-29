use crate::config::read_config_file;

pub fn get_editor() -> String {
    let config = read_config_file().unwrap();

    let editor = config.editor.editor_name;
    editor
}
