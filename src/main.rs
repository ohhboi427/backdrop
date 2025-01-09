use std::{
    collections::VecDeque,
    env,
    fs::{self, File},
    io::{self, Read, Write},
    path::{Path, PathBuf},
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

fn setup<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = path.as_ref();

    if !path.exists() {
        fs::create_dir_all(&path)?;
    }

    let env_path = path.join(".env");
    if !env_path.exists() {
        println!(
            "You must set the Unsplash Access Key in {}",
            env_path.display()
        );

        fs::copy(".env.example", &env_path)?;
    }

    dotenvy::from_path(env_path)?;

    Ok(())
}

fn configure<P: AsRef<Path>>(path: P) -> Result<Config> {
    let config_path = path.as_ref().join("config.json");

    let config = if let Ok(mut file) = File::open(&config_path) {
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        serde_json::from_str(&contents)?
    } else {
        let config = Config::default();
        let content = serde_json::to_string_pretty(&config)?;

        let mut file = File::create(&config_path)?;
        file.write_all(content.as_bytes())?;

        config
    };

    Ok(config)
}

#[tokio::main]
async fn main() -> Result<()> {
    let path = dirs::config_dir().unwrap().join("Backdrop");

    setup(&path)?;
    let config = configure(&path)?;

    download_photos(&config).await?;
    delete_old_photos(&config)?;

    Ok(())
}
