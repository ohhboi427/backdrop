#![windows_subsystem = "windows"]

use std::{
    collections::VecDeque,
    fs, io,
    path::{Path, PathBuf},
    time::UNIX_EPOCH,
};

use bytes::Bytes;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::task::JoinSet;

use backdrop::{unsplash, Client, Download, Fetch, Photo};

#[derive(Debug, Error)]
enum Error {
    #[error("{0}")]
    Io(#[from] io::Error),

    #[error("{0}")]
    Unsplash(#[from] unsplash::Error),

    #[error("A default configuration file has been created, please review it before proceeding")]
    RequiresConfigure,
}

type Result<T> = core::result::Result<T, Error>;

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
    let client = Client::new_from_env()?;

    let photos = client.fetch_photos(&config.fetch).await?;

    let mut tasks = JoinSet::<backdrop::Result<(Photo, Bytes)>>::new();
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

        fs::write(&path, &data)?;
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

fn configure<P: AsRef<Path>>(config_folder: P) -> Result<Config> {
    let config_folder = config_folder.as_ref();

    if !config_folder.exists() {
        fs::create_dir_all(&config_folder)?;
    }

    let env_path = config_folder.join(".env");
    let config_path = config_folder.join("config.json");
    let requires_config = !env_path.exists() || !config_path.exists();

    if requires_config {
        if !env_path.exists() {
            fs::copy(".env.example", &env_path)?;
        }

        if !config_path.exists() {
            let config = Config::default();
            let content = serde_json::to_string_pretty(&config)
                .map_err(|err| Into::<io::Error>::into(err))?;

            fs::write(&config_path, &content)?;
        }

        return Err(Error::RequiresConfigure);
    }

    dotenvy::from_path(env_path).map_err(|err| match err {
        dotenvy::Error::Io(err) => err,

        _ => unreachable!(),
    })?;

    let config = {
        let content = fs::read_to_string(&config_path)?;

        serde_json::from_str(&content).map_err(|err| Into::<io::Error>::into(err))?
    };

    Ok(config)
}

#[tokio::main]
async fn main() {
    async fn run() -> Result<()> {
        let path = dirs::config_dir().unwrap().join("Backdrop");

        let config = configure(&path)?;

        download_photos(&config).await?;
        delete_old_photos(&config)?;

        Ok(())
    }

    if let Err(e) = run().await {
        eprintln!("{}", e);
    }
}
