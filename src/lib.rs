mod editor;
mod config;
mod args;
mod environment;
mod search;
mod install;
mod download;

use args::SandboxArgs;
use clap::Parser;
use search::search;
use serde_yaml::{Value, Mapping};

use environment::setup_environment;
use install::install_environment;

pub async fn run() { 
    let args = SandboxArgs::parse();

    if !args.search.is_empty() {
        search(args.search)
    }

    if !args.beach_type.is_empty() {
        setup_environment(args.beach_type)
    }

    if !args.install.is_empty() {
        install_environment(args.install).await;
    }
}

pub fn get_projects_list() -> Mapping {
    let sandbox_templates_path = "../sandbox-templates/sandbox-templates.yml";
    let file_contents = std::fs::read_to_string(sandbox_templates_path).unwrap();
    let templates: Value = serde_yaml::from_str(&file_contents).unwrap();

    if let Some(languages) = templates["languages"].as_mapping() {
        for (_language_name, projects) in languages {
            if let Some(projects_list) = projects.as_mapping() {
                return projects_list.to_owned()
            } 
        }
    }
    Mapping::new()
}

pub fn get_title(id: String) -> String {
    for (project_id, project_object) in get_projects_list() {
        if project_id.as_str().unwrap() == id {
            return project_object["title"].as_str().unwrap().to_string();
        } 
    } 
    String::new()
}

pub fn get_description(id: String) -> String {
    for (project_id, project_object) in get_projects_list() {
        if project_id.as_str().unwrap() == id {
            return project_object["description"].as_str().unwrap().to_string();
        } 
    } 
    String::new()
}

pub fn get_path(id: String) -> String {
    for (project_id, project_object) in get_projects_list() {
        if project_id.as_str().unwrap() == id {
            return project_object["path"].as_str().unwrap().to_string();
        } 
    } 
    String::new()
}

pub fn get_keywords(id: String) -> String {
    for (project_id, project_object) in get_projects_list() {
        if project_id.as_str().unwrap() == id {
            return project_object["path"].as_str().unwrap().to_string();
        } 
    }
    String::new()
}

pub fn get_project_object(id: String) -> Value {
    for (project_id, project_object) in get_projects_list() {
        if project_id.as_str().unwrap() == id {
            return project_object.to_owned()
        } 
    }
    Value::Null
}

pub fn id_is_valid(id: String) -> bool {
    for (project_id, _) in get_projects_list() {
        if project_id.as_str().unwrap() == id {
            return true
        }
    }
    false
}
