use crate::install::in_system;
use crate::get_templates_mapping;

pub fn search(query: String) {
    println!("Environments That Match Query {}\n", query);

    search_environment(query);

    println!("\nInstall any of these with sandbox install <ENVIRONMENT>")
}

pub fn search_environment(query: String) {
    let _sandbox_url = "https://raw.githubusercontent.com/the-sandbox-project/sandbox-templates/master/sandbox-templates.yml";
    for (_language, project) in get_templates_mapping() {
        for (project_id, project_object) in project.as_mapping().unwrap() {
            let keywords = project_object["keywords"].as_str().unwrap_or("");
            let keywords_list: Vec<&str> = keywords.split_whitespace().collect();

            for keyword in keywords_list {
                if keyword == query {
                    let title = project_object["title"].as_str().unwrap_or("Title not Found");
                    let description = project_object["description"].as_str().unwrap_or("Description not Found");
                    let id = project_id.as_str().unwrap().to_string();

                    let installed = match in_system(id.clone()) {
                        true => "✅",
                        false => "❌"
                    };

                    println!("     {} {} ({}) - {}", installed, title, id, description)
                }
            }
        }
    }
}

