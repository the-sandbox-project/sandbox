use crate::{get_projects_list, install::in_system};

pub fn search(query: String) {
    println!("Environments That Match Query {}\n", query);

    search_environment(query);

    println!("\nInstall any of these with sandbox install <ENVIRONMENT>")
}

pub fn search_environment(query: String) {
    let _sandbox_url = "https://raw.githubusercontent.com/the-sandbox-project/sandbox-templates/master/sandbox-templates.yml";
    for (project_id, project) in get_projects_list() {
        println!("project_id {:#?}", project_id);
        let keywords = project["keywords"].as_str().unwrap_or("");
        let keywords_list: Vec<&str> = keywords.split_whitespace().collect();
        dbg!(&keywords_list);

        for keyword in keywords_list {
            if keyword == query {
                let title = project["title"].as_str().unwrap_or("Title not Found");
                let description = project["description"].as_str().unwrap_or("Description not Found");
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

