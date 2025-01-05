pub mod unsplash;

use std::{env, fs::File, io::Write};

use unsplash::{Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().unwrap();
    let api_key = env::var("UNSPLASH_API_KEY").unwrap();

    let client = Client::new(&api_key)?;

    let topic = client.find_topic("nature").await?;

    let photos = client.fetch_photos(&topic).await?;

    let images = client.download_photos(photos).await;
    for (photo, data) in images {
        let mut file = File::create(format!("{}.png", photo.id())).unwrap();
        file.write_all(data?.as_ref()).unwrap()
    }

    Ok(())
}
