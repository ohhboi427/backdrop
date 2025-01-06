pub mod unsplash;

use std::{env, fs::File, io::Write};

use tokio::task::JoinSet;

use unsplash::{Client, Quality, Result};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().unwrap();
    let api_key = env::var("UNSPLASH_API_KEY").unwrap();

    let client = Client::new(&api_key)?;

    let topic = client.find_topic("nature").await?;
    let photos = client.fetch_photos(&topic, 10).await?;

    std::fs::create_dir("photos").unwrap();

    let mut tasks = JoinSet::<Result<()>>::new();
    for photo in photos {
        let client = client.clone();

        tasks.spawn(async move {
            let data = client
                .download_photo(&photo, Quality::Custom(1920, 1080))
                .await?;

            let mut file = File::create(format!("photos/{}.png", photo.id())).unwrap();
            file.write_all(&data).unwrap();

            Ok(())
        });
    }

    tasks.join_all().await;

    Ok(())
}
