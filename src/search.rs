use std::fs;

use serde_yaml::Value;

pub fn search(query: String) {
    println!("Environments That Match Query {}\n", query);

    search_environment(query);

    println!("\nInstall any of these with sandbox install <ENVIRONMENT>")
}

pub fn search_environment(query: String) {
    let _sandbox_url = "https://raw.githubusercontent.com/the-sandbox-project/sandbox-templates/master/sandbox-templates.yml";
    let sandbox_templates_path = "../sandbox-templates/sandbox-templates.yml";

    let file_contents = std::fs::read_to_string(sandbox_templates_path).unwrap();
    let templates: Value = serde_yaml::from_str(&file_contents).unwrap();

    if let Some(languages) = templates["languages"].as_mapping() {
        for (_language_name, projects) in languages {
            if let Some(projects_list) = projects.as_mapping() {
                for (project_id, project) in projects_list {
                    let keywords = project["keywords"].as_str().unwrap_or("");
                    let keywords_list: Vec<&str> = keywords.split_whitespace().collect();

                    for keyword in keywords_list {
                        if keyword == query {
                            let title = project["title"].as_str().unwrap_or("Title not Found");
                            let description = project["description"].as_str().unwrap_or("Description not Found");
                            let path = format!("/usr/share/sandbox/beaches/{}", project["path"].as_str().unwrap_or("Path not Found"));

                            let installed = match fs::read(path) {
                                Ok(_) => "✅",
                                Err(_) => "❌"
                            };

                            println!("     {} {} ({}) - {}", installed, title, project_id.as_str().unwrap(), description)
                        }
                    }
                }
            }
        }
    }
}

