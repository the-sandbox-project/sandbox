use colored::Colorize;

use crate::id_is_valid;
use crate::download::install;
use crate::in_system;

pub async fn install_environment(id: String) {
    if id_is_valid(id.clone()).await {
        if in_system(id.clone()).await {
            println!("You already have {} installed on your system!", id.bright_green());
        } else {
            install(id.clone()).await.unwrap();
        }
    } else {
           println!("The environment ({}) does {} exist!", id.bright_green(), "not".red())
    }
}

