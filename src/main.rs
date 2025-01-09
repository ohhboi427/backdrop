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
use unsplash::{Client, Photo, Quality};

#[derive(Serialize, Deserialize)]
struct Properties {
    folder: PathBuf,
    topic: String,
    count: u32,
    max_size: u64,
}

impl Default for Properties {
    fn default() -> Self {
        let folder = PathBuf::from(env::var("USERPROFILE").unwrap_or_default())
            .join("Pictures")
            .join("Backdrop");

        Self {
            folder,
            topic: "nature".to_owned(),
            count: 10,
            max_size: 100_000_000,
        }
    }
}

async fn download_photos(properties: &Properties) -> Result<()> {
    let api_key = env::var("UNSPLASH_API_KEY")?;

    let client = Client::new(&api_key)?;

    let topic = client.find_topic(&properties.topic).await?;
    let photos = client.fetch_photos(&topic, properties.count).await?;

    fs::create_dir_all(&properties.folder)?;

    let mut tasks = JoinSet::<unsplash::Result<(Photo, Bytes)>>::new();
    for photo in photos {
        let client = client.clone();

        tasks.spawn(async move {
            let data = client
                .download_photo(&photo, Quality::Custom(1920, 1080))
                .await?;

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

    let config_path = Path::new("config.toml");

    let properties = match File::open(config_path) {
        Ok(mut file) => {
            let mut contents = Default::default();
            file.read_to_string(&mut contents)?;

            toml::from_str(&contents)?
        }

        Err(_) => {
            let properties = Properties::default();
            let contents = toml::to_string_pretty(&properties)?;

            let mut file = File::create(config_path)?;
            file.write_all(contents.as_bytes())?;

            properties
        }
    };

    download_photos(&properties).await?;
    delete_old_photos(&properties)?;

    Ok(())
}
