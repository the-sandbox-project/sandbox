mod editor;
mod config;
mod args;
mod environment;
mod search;
mod install;
mod uninstall;
mod download;
mod cache;
mod new;

use std::error::Error;
use std::path::Path;
use std::env;

use args::SandboxArgs;
use search::search;
use install::install_environment;
use uninstall::uninstall_environment;
use new::create_new_environment;
use cache::clear_cache;

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

    if !args.uninstall.is_empty() {
        uninstall_environment(args.uninstall).await;
    }

    if args.clearcache {
        clear_cache().await;
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
 
pub async fn get_title(id: impl Into<String>) -> String {
    let id = id.into();

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

pub async fn get_description(id: impl Into<String>) -> String {
    let id = id.into();

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

pub async fn get_path(id: impl Into<String>) -> String {
    let id = id.into();

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

pub async fn get_keywords(id: impl Into<String>) -> String {
    let id = id.into();

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

pub async fn get_project_object(id: impl Into<String>) -> Value {
    let id = id.into();

    for (_language, project_list) in get_templates_mapping().await.unwrap() {
        if let Some(project) = project_list.as_mapping() {
            if let Some(list) = project.get(&id) {
                return list.to_owned()
            };
        }
    } 
    Value::Null
}

pub async fn id_is_valid(id: impl Into<String>) -> bool{
    let id = id.into();

    for (_language, project_list) in get_templates_mapping().await.unwrap() {
        if let Some(project) = project_list.as_mapping() {
            if let Some(_) = project.get(&id) {
                return true
            };
        }
    } 
    false
}

pub async fn in_system(id: impl Into<String>) -> bool {
    let id = id.into();
    
    let base_path = get_beaches_path();        

    let path = get_path(id.clone()).await;
    let environment_path = path.split("/").collect::<Vec<&str>>()[0].to_owned() + "/" + &id;
    let formatted_path = format!("{}{}", base_path, environment_path);

    let path = Path::new(&formatted_path);

    if path.exists() && path.is_dir() {
        true
    } else {
        false
    }
}

pub fn get_beaches_path() -> String {
    match env::consts::OS {
        "windows" => {
            let appdata = env::var("APPDATA").unwrap();
            let beaches_path = format!("{}/sandbox/beaches/", appdata);
            beaches_path
        }
        _ => "/usr/share/sandbox/beaches/".to_string(), 
    }
}

pub fn get_cache_path() -> String {
    match env::consts::OS {
        "windows" => {
            let appdata = env::var("LOCALAPPDATA").unwrap();
            let beaches_path = format!("{}/Temp/sandbox/", appdata);
            beaches_path
        }
        _ => { 
            let home_path = env::var("HOME").unwrap();
            let cache_path = format!("{}/.cache/sandbox/", home_path);
            cache_path
        }
    }
}
