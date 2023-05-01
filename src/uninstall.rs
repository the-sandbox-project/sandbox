use std::env;
use std::io::{stdin, stdout, Write};

use crate::{get_path, id_is_valid, in_system};

use colored::Colorize;
use tokio::fs;

pub async fn uninstall_environment(environment: impl Into<String>) {
    let environment = environment.into();

    if id_is_valid(&environment).await {
        if in_system(&environment).await {
            uninstall(environment).await
        } else {
            println!(
                "You do {} have {} installed! Install it with:\nsandbox --install {}",
                "not".red(),
                environment.bright_green(),
                environment.bright_green()
            )
        }
    } else {
        println!("The environment ({}) does {} exist! You can search for an environment with\nsandbox --search {}", environment.bright_green(), "not".red(), environment.bright_green());
    }
}

pub async fn uninstall(environment: impl Into<String>) {
    let environment = environment.into();

    let base_path = match env::consts::OS {
        "windows" => {
            let appdata = std::env::var("appdata").unwrap();
            let beaches_path = format!("{}/sandbox/beaches/", appdata);
            beaches_path
        }
        _ => "/usr/share/sandbox/beaches/".to_string(),
    };

    let path = get_path(environment.clone()).await;
    let environment_path = path.split("/").collect::<Vec<&str>>()[0].to_owned() + "/" + &environment;

    let formatted_path = format!("{}{}", base_path, environment_path);

    let mut answer = String::new();

    print!(
        "Are you sure you want to delete {}? [{}/{}] ",
        &environment.blue(),
        "Y".bright_green(),
        "n".red()
    );

    stdout().lock().flush().unwrap();

    stdin().read_line(&mut answer).unwrap();

    answer = answer.trim().into();
    
    if answer == "Y" {
        match fs::remove_dir_all(formatted_path).await {
            Ok(_) => {
                println!("Successfully Deleted {} from System!", environment.bright_green())
            },
            Err(err) => {
                println!("There was an {} Deleting {}!\n{}", "Erorr".red(), environment.blue(), err)
            }
        }
    }
}
