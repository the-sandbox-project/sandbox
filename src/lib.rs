mod editor;
mod config;
mod args;
mod environment;
mod search;
mod install;
mod download;

use std::fs;

use args::SandboxArgs;
use search::search;
use environment::setup_environment;
use install::install_environment;

use clap::Parser;
use serde_yaml::{Value, Mapping};

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

pub fn get_templates_mapping() -> Mapping {
    let sandbox_templates_path = "../sandbox-templates/sandbox-templates.yml";
    let file_contents = fs::read_to_string(sandbox_templates_path).unwrap();
    let templates: Value = serde_yaml::from_str(&file_contents).unwrap();

    if let Some(languages) = templates["languages"].as_mapping() {
        return languages.to_owned()
    }
    Mapping::new()
}
 
pub fn get_title(id: String) -> String {
    for (_language, project_list) in get_templates_mapping() {
        if let Some(project) = project_list.as_mapping() {
            if let Some(list) = project.get(&id) {
                let path = list["title"].as_str().unwrap().to_string();
                return path
            };
        }
    } 
    String::new()
}

pub fn get_description(id: String) -> String {
    for (_language, project_list) in get_templates_mapping() {
        if let Some(project) = project_list.as_mapping() {
            if let Some(list) = project.get(&id) {
                let path = list["description"].as_str().unwrap().to_string();
                return path
            };
        }
    } 
    String::new()
}

pub fn get_path(id: String) -> String {
    for (_language, project_list) in get_templates_mapping() {
        if let Some(project) = project_list.as_mapping() {
            if let Some(list) = project.get(&id) {
                let path = list["path"].as_str().unwrap().to_string();
                return path
            };
        }
    } 
    String::new()
}

pub fn get_keywords(id: String) -> String {
    for (_language, project_list) in get_templates_mapping() {
        if let Some(project) = project_list.as_mapping() {
            if let Some(list) = project.get(&id) {
                let path = list["keywords"].as_str().unwrap().to_string();
                return path
            };
        }
    } 
    String::new()
}

pub fn get_project_object(id: String) -> Value {
    for (_language, project_list) in get_templates_mapping() {
        if let Some(project) = project_list.as_mapping() {
            if let Some(list) = project.get(&id) {
                return list.to_owned()
            };
        }
    } 
    Value::Null
}

pub fn id_is_valid(id: String) -> bool{
    for (_language, project_list) in get_templates_mapping() {
        if let Some(project) = project_list.as_mapping() {
            if let Some(_) = project.get(&id) {
                return true
            };
        }
    } 
    false
}
