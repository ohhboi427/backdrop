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
struct Properties {
    folder: PathBuf,
    max_size: u64,
    fetch: Fetch,
    download: Download,
}

impl Default for Properties {
    fn default() -> Self {
        let folder = PathBuf::from(env::var("USERPROFILE").unwrap_or_default())
            .join("Pictures")
            .join("Backdrop");

        Self {
            folder,
            max_size: 100_000_000,
            fetch: Default::default(),
            download: Default::default(),
        }
    }
}

async fn download_photos(properties: &Properties) -> Result<()> {
    let api_key = env::var("UNSPLASH_API_KEY")?;

    let client = Client::new(&api_key)?;

    let photos = client.fetch_photos(&properties.fetch).await?;

    fs::create_dir_all(&properties.folder)?;

    let mut tasks = JoinSet::<unsplash::Result<(Photo, Bytes)>>::new();
    for photo in photos {
        let client = client.clone();
        let download = properties.download.clone();

        tasks.spawn(async move {
            let data = client.download_photo(&photo, &download).await?;

            Ok((photo, data))
        });
    }

    let photos = tasks.join_all().await;
    for photo in photos {
        let (photo, data) = photo?;

        let path = properties.folder.join(format!("{}.png", photo.id()));

        let mut file = File::create(path)?;
        file.write_all(&data)?
    }

    Ok(())
}

fn delete_old_photos(properties: &Properties) -> io::Result<()> {
    let mut files: Vec<_> = properties
        .folder
        .read_dir()?
        .filter_map(|file| file.ok())
        .collect();

    let mut size: u64 = files
        .iter()
        .filter_map(|file| file.metadata().ok())
        .map(|metadata| metadata.len())
        .sum();

    if size <= properties.max_size {
        return Ok(());
    }

    files.sort_by_key(|file| {
        file.metadata()
            .ok()
            .and_then(|metadata| metadata.created().ok())
            .unwrap_or(UNIX_EPOCH)
    });

    let mut files = VecDeque::from(files);
    while size > properties.max_size {
        let file = files.pop_front().unwrap();

        fs::remove_file(file.path())?;
        size -= file.metadata()?.len();
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;

    let config_path = Path::new("config.json");

    let properties = match File::open(config_path) {
        Ok(mut file) => {
            let mut contents = Default::default();
            file.read_to_string(&mut contents)?;

            serde_json::from_str(&contents)?
        }

        Err(_) => {
            let properties = Properties::default();
            let contents = serde_json::to_string_pretty(&properties)?;

            let mut file = File::create(config_path)?;
            file.write_all(contents.as_bytes())?;

            properties
        }
    };

    download_photos(&properties).await?;
    delete_old_photos(&properties)?;

    Ok(())
}
