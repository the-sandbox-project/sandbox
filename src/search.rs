use crate::install::in_system;
use crate::get_templates_mapping;

use colored::Colorize;

pub async fn search(query: String) {
    let matching_environments_exist = search_environment(query.clone()).await;

    if matching_environments_exist {
        println!("\nEnvironments That Match Query {}", query.bright_green());
        println!("Install any of these with sandbox install <ENVIRONMENT>")
    } else {
        println!("{} Environments Found for Query: {}", "No".red(), query.bright_green())
    }
}

pub async fn search_environment(query: String) -> bool {
    let mut matching_environments_exist = false;

    for (_language, project) in get_templates_mapping().await.unwrap() {
        for (project_id, project_object) in project.as_mapping().unwrap() {
            let keywords = project_object["keywords"].as_str().unwrap_or("");
            let keywords_list: Vec<&str> = keywords.split_whitespace().collect();

            for keyword in keywords_list {
                if keyword == query {
                    let title = project_object["title"].as_str().unwrap_or("Title not Found");
                    let description = project_object["description"].as_str().unwrap_or("Description not Found");
                    let id = project_id.as_str().unwrap().to_string();

                    let installed = match in_system(id.clone()).await {
                        true => "✅".bright_green(),
                        false => "❌".red()
                    };

                    matching_environments_exist = true;

                    println!("     {} {} ({}) - {}", installed, title, id.blue(), description);

                }
            }
        }
    }
    matching_environments_exist
}

