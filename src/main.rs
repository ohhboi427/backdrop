use std::{
    env,
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
    time::UNIX_EPOCH,
};

use anyhow::Result;
use bytes::Bytes;
use tokio::task::JoinSet;

mod unsplash;
use unsplash::{Client, Photo, Quality};

async fn fetch_photos<P: AsRef<Path> + Send + Clone>(folder: P) -> Result<()> {
    let api_key = env::var("UNSPLASH_API_KEY")?;

    let client = Client::new(&api_key)?;

    let topic = client.find_topic("nature").await?;
    let photos = client.fetch_photos(&topic, 10).await?;

    fs::create_dir_all(folder.as_ref())?;

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

        let path = folder.as_ref().join(format!("{}.png", photo.id()));

        let mut file = File::create(path)?;
        file.write_all(&data)?
    }

    Ok(())
}

fn delete_old_photos<P: AsRef<Path>>(folder: P, max_size: u64) -> io::Result<()> {
    let mut files: Vec<_> = folder
        .as_ref()
        .read_dir()?
        .filter_map(|file| file.ok())
        .collect();

    let mut size: u64 = files
        .iter()
        .filter_map(|file| file.metadata().ok())
        .map(|metadata| metadata.len())
        .sum();

    if size <= max_size {
        return Ok(());
    }

    files.sort_by_key(|file| {
        file.metadata()
            .ok()
            .and_then(|metadata| metadata.created().ok())
            .unwrap_or(UNIX_EPOCH)
    });

    while size > max_size {
        let file = files.remove(files.len() - 1);

        fs::remove_file(file.path())?;

        size -= file.metadata()?.len();
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;

    let folder = PathBuf::from(env::var("USERPROFILE")?)
        .join("Pictures")
        .join("Backdrop");

    fetch_photos(&folder).await?;
    delete_old_photos(&folder, 100_000_000)?;

    Ok(())
}
