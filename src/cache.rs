use std::path::Path;

use tokio::fs;
use colored::Colorize;

use crate::{get_path, get_cache_path};

pub async fn ensure_installed() {
    let cache_path = get_cache_path();

    fs::create_dir_all(cache_path).await.unwrap();
}

pub async fn clear_cache() {
    let cache_path = get_cache_path();

    let cache_folder_is_empty = fs::read_dir(&cache_path).await.unwrap().next_entry().await.unwrap().is_none();

    if cache_folder_is_empty {
        println!("The Install Cache is {}!", "Empty".bright_green())
    } else {
        fs::remove_dir_all(&cache_path).await.unwrap();
        fs::create_dir(&cache_path).await.unwrap();

        println!("{} Cleared The Install Cache!", "Successfully".bright_green())
    }
}

pub async fn in_cache(id: impl Into<String>) -> bool {
    let id = id.into();

    let cache_path = get_cache_path();

    let environment_path = get_path(&id).await;
    let tar_name = environment_path.split('/').last().unwrap();

    let formatted_cache_path = format!("{}{}",  cache_path, tar_name);
    let tar_in_cache = Path::new(&formatted_cache_path).exists();

    tar_in_cache
}
