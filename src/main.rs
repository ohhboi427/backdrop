use std::time::UNIX_EPOCH;
use std::{
    env,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use tokio::task::JoinSet;

mod unsplash;
use unsplash::{Client, Quality, Result};

async fn fetch_photos<P: AsRef<Path> + Send + Clone>(folder: P) -> Result<()> {
    let api_key = env::var("UNSPLASH_API_KEY").unwrap();

    let client = Client::new(&api_key)?;

    let topic = client.find_topic("nature").await?;
    let photos = client.fetch_photos(&topic, 10).await?;

    fs::create_dir_all(folder.as_ref()).unwrap();

    let mut tasks = JoinSet::<Result<()>>::new();
    for photo in photos {
        let client = client.clone();
        let path = folder.as_ref().join(format!("{}.png", &photo.id()));

        tasks.spawn(async move {
            let data = client
                .download_photo(&photo, Quality::Custom(1920, 1080))
                .await?;

            let mut file = File::create(path).unwrap();
            file.write_all(&data).unwrap();

            Ok(())
        });
    }

    tasks.join_all().await;

    Ok(())
}

fn delete_old_photos<P: AsRef<Path>>(folder: P, max_size: u64) {
    let mut files: Vec<_> = folder
        .as_ref()
        .read_dir()
        .unwrap()
        .filter_map(|file| file.ok())
        .collect();

    let mut size: u64 = files
        .iter()
        .map(|file| file.metadata().unwrap().len())
        .sum();

    if size <= max_size {
        return;
    }

    files.sort_by_key(|file| file.metadata().unwrap().created().unwrap_or(UNIX_EPOCH));

    while size > max_size {
        let file = files.remove(files.len() - 1);

        fs::remove_file(file.path()).unwrap();

        size -= file.metadata().unwrap().len();
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().unwrap();

    let folder = PathBuf::from(env::var("USERPROFILE").unwrap())
        .join("Pictures")
        .join("Backdrop");

    fetch_photos(&folder).await?;
    delete_old_photos(&folder, 100_000_000);

    Ok(())
}
