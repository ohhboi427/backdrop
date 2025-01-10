use std::{
    collections::VecDeque,
    fs::{self, File},
    io::{self, Read, Write},
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

fn configure<P: AsRef<Path>>(config_folder: P) -> Result<Config> {
    let config_folder = config_folder.as_ref();

    if !config_folder.exists() {
        fs::create_dir_all(&config_folder)?;
    }

    let env_path = config_folder.join(".env");
    if !env_path.exists() {
        println!(
            "You must set the Unsplash Access Key in {}",
            env_path.display()
        );

        fs::copy(".env.example", &env_path)?;
    }

    dotenvy::from_path(env_path).map_err(|err| match err {
        dotenvy::Error::Io(err) => err,

        _ => unreachable!(),
    })?;

    let config_path = config_folder.join("config.json");
    let config = match File::open(&config_path) {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            serde_json::from_str(&contents).map_err(|err| Into::<io::Error>::into(err))?
        }

        Err(_) => {
            println!(
                "You can change the default options for fetching and downloading photos in {}",
                config_path.display()
            );

            let config = Config::default();
            let content = serde_json::to_string_pretty(&config)
                .map_err(|err| Into::<io::Error>::into(err))?;

            let mut file = File::create(&config_path)?;
            file.write_all(content.as_bytes())?;

            config
        }
    };

    Ok(config)
}

#[tokio::main]
async fn main() -> Result<()> {
    let path = dirs::config_dir().unwrap().join("Backdrop");

    let config = configure(&path)?;

    download_photos(&config).await?;
    delete_old_photos(&config)?;

    Ok(())
}
