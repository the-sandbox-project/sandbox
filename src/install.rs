use serde_yaml::Value;
use std::fs;

pub fn install_environment(id: String) {
    let (exists, path) = environment_exists(id.clone());
    if exists {
        if in_system(path) {
            println!("You already have {} installed on your system!", id)
        } else {
            // TODO: Install the thing
            println!("I gotta do the installing")
        }
    } else {
        println!("The environment {} does not exist!", id)
    }
}

pub fn environment_exists(id: String) -> (bool, String) {
    let sandbox_templates_path = "../sandbox-templates/sandbox-templates.yml";
    let file_contents = std::fs::read_to_string(sandbox_templates_path).unwrap();
    let templates: Value = serde_yaml::from_str(&file_contents).unwrap();

    if let Some(languages) = templates["languages"].as_mapping() {
        for (_language_name, projects) in languages {
            if let Some(projects_list) = projects.as_mapping() {
                for (project_id, project) in projects_list {
                    if project_id.as_str().unwrap() == id {
                        let path = project["path"].as_str().unwrap();
                        let path_string = path.to_string();
                        return (true, path_string)
                    }
                } 
            } 
        }
    } 
    (false, String::new())
}

pub fn in_system(path: String) -> bool {
    let formatted_path = format!("/usr/share/sandbox/beaches/{}", path);

    let installed = match fs::read(formatted_path) {
        Ok(_) => true,
        Err(_) => false
    };

    installed
}
