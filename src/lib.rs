mod editor;
mod config;
mod args;
mod environment;
mod search;
mod install;
mod download;
mod new;

use std::error::Error;

use args::SandboxArgs;
use search::search;
use install::install_environment;
use new::create_new_environment;

use clap::Parser;
use serde_yaml::{Value, Mapping};
use reqwest::Url;

pub async fn run() { 
    let args = SandboxArgs::parse();

    if !args.new.is_empty() {
        create_new_environment(args.new).await
    }

    if !args.search.is_empty() {
        search(args.search).await
    }

    if !args.install.is_empty() {
        install_environment(args.install).await;
    }
}

pub async fn get_templates_mapping() -> Result<Mapping, Box<dyn Error>> {
    let url = Url::parse("https://raw.githubusercontent.com/the-sandbox-project/sandbox-templates/master/sandbox-templates.yml")?;
    let response = reqwest::get(url).await?.text().await?;

    let templates: Value = serde_yaml::from_str(response.as_str()).unwrap();

    if let Some(languages) = templates["languages"].as_mapping() {
        Ok(languages.to_owned())
    } else {
        Ok(Mapping::new())
    }
}
 
pub async fn get_title(id: String) -> String {
    for (_language, project_list) in get_templates_mapping().await.unwrap() {
        if let Some(project) = project_list.as_mapping() {
            if let Some(list) = project.get(&id) {
                let path = list["title"].as_str().unwrap().to_string();
                return path
            };
        }
    } 
    String::new()
}

pub async fn get_description(id: String) -> String {
    for (_language, project_list) in get_templates_mapping().await.unwrap() {
        if let Some(project) = project_list.as_mapping() {
            if let Some(list) = project.get(&id) {
                let path = list["description"].as_str().unwrap().to_string();
                return path
            };
        }
    } 
    String::new()
}

pub async fn get_path(id: String) -> String {
    for (_language, project_list) in get_templates_mapping().await.unwrap() {
        if let Some(project) = project_list.as_mapping() {
            if let Some(list) = project.get(&id) {
                let path = list["path"].as_str().unwrap().to_string();
                return path
            };
        }
    } 
    String::new()
}

pub async fn get_keywords(id: String) -> String {
    for (_language, project_list) in get_templates_mapping().await.unwrap() {
        if let Some(project) = project_list.as_mapping() {
            if let Some(list) = project.get(&id) {
                let path = list["keywords"].as_str().unwrap().to_string();
                return path
            };
        }
    } 
    String::new()
}

pub async fn get_project_object(id: String) -> Value {
    for (_language, project_list) in get_templates_mapping().await.unwrap() {
        if let Some(project) = project_list.as_mapping() {
            if let Some(list) = project.get(&id) {
                return list.to_owned()
            };
        }
    } 
    Value::Null
}

pub async fn id_is_valid(id: String) -> bool{
    for (_language, project_list) in get_templates_mapping().await.unwrap() {
        if let Some(project) = project_list.as_mapping() {
            if let Some(_) = project.get(&id) {
                return true
            };
        }
    } 
    false
}
