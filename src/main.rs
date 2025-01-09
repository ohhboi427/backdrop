use std::{
    collections::VecDeque,
    env,
    fs::{self, File},
    io::{self, Read, Write},
    path::PathBuf,
    time::UNIX_EPOCH,
};

use anyhow::Result;
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use tokio::task::JoinSet;

mod unsplash;
use unsplash::{Client, Download, Fetch, Photo};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Config {
    folder: PathBuf,
    max_size: u64,
    fetch: Fetch,
    download: Download,
}

impl Default for Config {
    fn default() -> Self {
        let folder = dirs::picture_dir().unwrap().join("Backdrop");

        Self {
            folder,
            max_size: 100_000_000,
            fetch: Default::default(),
            download: Default::default(),
        }
    }
}

async fn download_photos(config: &Config) -> Result<()> {
    let api_key = env::var("UNSPLASH_API_KEY")?;
    let client = Client::new(&api_key)?;

    let photos = client.fetch_photos(&config.fetch).await?;

    let mut tasks = JoinSet::<unsplash::Result<(Photo, Bytes)>>::new();
    for photo in photos {
        let client = client.clone();
        let download = config.download.clone();

        tasks.spawn(async move {
            let data = client.download_photo(&photo, &download).await?;

            Ok((photo, data))
        });
    }

    fs::create_dir_all(&config.folder)?;

    let photos = tasks.join_all().await;
    for photo in photos {
        let (photo, data) = photo?;

        let path = config.folder.join(format!("{}.png", photo.id()));

        let mut file = File::create(path)?;
        file.write_all(&data)?
    }

    Ok(())
}

fn delete_old_photos(config: &Config) -> io::Result<()> {
    let mut files: Vec<_> = config
        .folder
        .read_dir()?
        .filter_map(|file| file.ok())
        .collect();

    let mut size: u64 = files
        .iter()
        .filter_map(|file| file.metadata().ok())
        .map(|metadata| metadata.len())
        .sum();

    if size <= config.max_size {
        return Ok(());
    }

    files.sort_by_key(|file| {
        file.metadata()
            .ok()
            .and_then(|metadata| metadata.created().ok())
            .unwrap_or(UNIX_EPOCH)
    });

    let mut files = VecDeque::from(files);
    while size > config.max_size {
        let file = files.pop_front().unwrap();

        fs::remove_file(file.path())?;
        size -= file.metadata()?.len();
    }

    Ok(())
}

fn config() -> Result<Config> {
    const CONFIG_PATH: &'static str = "config.json";

    fn read_config() -> Result<Config> {
        let mut file = File::open(CONFIG_PATH)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Ok(serde_json::from_str(&contents)?)
    }

    fn write_config() -> Result<Config> {
        let config = Config::default();
        let contents = serde_json::to_string_pretty(&config)?;

        let mut file = File::create(CONFIG_PATH)?;
        file.write_all(contents.as_bytes())?;

        Ok(config)
    }

    read_config().or_else(|_| write_config())
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;

    let config = config()?;
    download_photos(&config).await?;
    delete_old_photos(&config)?;

    Ok(())
}
