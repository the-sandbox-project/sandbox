use std::error::Error;
use std::fs::File;
use std::io;
use std::path::Path;

use reqwest::Client;
use flate2::read::GzDecoder;
use tar::Archive;
use tokio::fs;

use crate::get_path;

pub async fn download_environment(id: String) -> Result<(), Box<dyn Error>> {
    let base_path = "/usr/share/sandbox/beaches/";
    let environment_path = get_path(id.clone());

    let download_url = format!("https://github.com/the-sandbox-project/sandbox-templates/raw/master/{}", environment_path);
    let download_path = format!("{}{}", base_path, environment_path);

    let client = Client::new();
    let response = client.get(&download_url).send().await?;

    if response.status().is_success() {
        let language_path = Path::new(&download_path).parent().unwrap().to_str().unwrap();
        fs::create_dir_all(language_path).await?;

        let mut file = File::create(&download_path)?;
        let content = response.bytes().await?;

        io::copy(&mut content.as_ref(), &mut file)?;

        let unzip_path = format!("{}/{}", language_path, id);
        dbg!(&unzip_path);

        let tar_gz = File::open(&download_path)?;
        let tar = GzDecoder::new(tar_gz);
        let mut archive = Archive::new(tar);

        archive.unpack(&unzip_path)?;
        fs::remove_file(&download_path).await?;
    }
    Ok(())
}

