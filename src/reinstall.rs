use crate::id_is_valid;
use crate::download::install;
use crate::uninstall::uninstall;

use colored::Colorize;

pub async fn reinstall_environment(id: impl Into<String>) {
    let id = id.into();

    if id_is_valid(&id).await {
        reinstall(&id).await;
    } else {
       println!("The environment ({}) does {} exist! You can search for an environment with\nsandbox --search {}", id.bright_green(), "not".red(), id.bright_green());
    }
}

pub async fn reinstall(id: impl Into<String>) {
    let id = id.into();

    uninstall(&id).await;
    install(&id).await.unwrap();
}
