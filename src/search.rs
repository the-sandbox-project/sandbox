use crate::install::in_system;
use crate::get_templates_mapping;

pub async fn search(query: String) {
    let matching_environments_exist = search_environment(query.clone()).await;

    if matching_environments_exist {
        println!("\nEnvironments That Match Query {}", query);
        println!("Install any of these with sandbox install <ENVIRONMENT>")
    } else {
        println!("No Environments Found for Query: {}", query)
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
                        true => "✅",
                        false => "❌"
                    };

                    matching_environments_exist = true;

                    println!("     {} {} ({}) - {}", installed, title, id, description);

                }
            }
        }
    }
    matching_environments_exist
}

