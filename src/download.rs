use std::error::Error;
use std::fs::File;
use std::io;
use std::path::Path;
use std::thread;
use std::time::Duration;
use std::fs;
use std::{cmp::min, fmt::Write};


use reqwest::Client;
use flate2::read::GzDecoder;
use tar::Archive;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use colored::Colorize;

use crate::get_beaches_path;
use crate::get_cache_path;
use crate::get_path;
use crate::cache::{self, in_cache};

pub async fn install(id: impl Into<String>) -> Result<(), Box<dyn Error>> {
    let id = id.into();

    let base_path = get_beaches_path();
    let environment_path = get_path(&id).await;

    let download_url = format!("https://github.com/the-sandbox-project/sandbox-templates/raw/master/{}", environment_path);
    let download_path = format!("{}{}", base_path, environment_path);

    let tar_name = environment_path.split('/').last().unwrap();
    
    cache::ensure_installed().await;
    let cache_path = get_cache_path();

    let formatted_cache_path = format!("{}{}",  cache_path, tar_name);

    if in_cache(&id).await {
        println!("{} found in {}! Installing from {}...", id.blue(), "Cache".bright_green(), "Cache".bright_green());

        let language_path = Path::new(&download_path).parent().unwrap().to_str().unwrap();

        let unzip_path = format!("{}/{}", language_path, id);

        fs::create_dir_all(&language_path)?;

        fs::copy(&formatted_cache_path, &download_path)?;

        let tar_gz = File::open(&download_path)?;
        let tar = GzDecoder::new(tar_gz);
        let mut archive = Archive::new(tar);

        archive.unpack(&unzip_path)?;

        File::create(&formatted_cache_path)?;

        fs::copy(&download_path, &formatted_cache_path)?;

        fs::remove_file(&download_path)?;

        println!("Installed {}! Test it out with: sandbox --new {}", id.bright_green(), id.bright_green());

        Ok(())
    } else {
        let client = Client::new();

        let response = client.get(download_url).send().await?;

        if response.status().is_success() {
            let language_path = Path::new(&download_path).parent().unwrap().to_str().unwrap();

            fs::create_dir_all(language_path)?;

            let mut file = File::create(&download_path)?;

            let mut downloaded = 0;
            let total_size = response.content_length().unwrap();

            let pb = ProgressBar::new(total_size);
            pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                .unwrap()
                .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
                .progress_chars("#>-"));

            while downloaded < total_size {
                let new = min(downloaded + 223211, total_size);
                downloaded = new;
                pb.set_position(new);
                thread::sleep(Duration::from_millis(12));
            }
            
            pb.finish_with_message("downloaded");

            let content = response.bytes().await?;

            io::copy(&mut content.as_ref(), &mut file)?;

            let unzip_path = format!("{}/{}", language_path, id);

            let tar_gz = File::open(&download_path)?;
            let tar = GzDecoder::new(tar_gz);
            let mut archive = Archive::new(tar);

            archive.unpack(&unzip_path)?;

            File::create(&formatted_cache_path)?;

            fs::copy(&download_path, &formatted_cache_path)?;

            fs::remove_file(&download_path)?;

            println!("Installed {}! Test it out with: sandbox --new {}", id.bright_green(), id.bright_green())
        }
        Ok(())
    } 
}

