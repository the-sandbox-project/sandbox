use crate::id_is_valid;
use crate::environment::open_environment;
use crate::in_system;

use colored::Colorize;

pub async fn create_new_environment(environment: String) {
    if id_is_valid(environment.clone()).await {
        if in_system(environment.clone()).await {
            open_environment(environment).await
        } else {
            println!("You do {} have {} installed! Install it with:\nsandbox --install {}", "not".red(), environment.bright_green(), environment.bright_green())
        }
    } else {
       println!("The environment ({}) does {} exist! You can search for an environment with\nsandbox --search {}", environment.bright_green(), "not".red(), environment.bright_green());
    }
}
