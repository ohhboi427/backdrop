pub mod unsplash;

use std::{env, fs::File, io::Write};

use unsplash::{Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().unwrap();
    let api_key = env::var("UNSPLASH_API_KEY").unwrap();

    let client = Client::new(&api_key)?;

    let topic = client.find_topic("nature").await?;
    println!("{:?}", topic);

    let photos = client.fetch_photos(&topic).await?;
    println!("{:?}", photos);

    let images = client.download_photos(&photos).await;
    for (index, image) in images.into_iter().enumerate() {
        let mut file = File::create(format!("photo{}.png", index)).unwrap();
        file.write_all(image?.as_ref()).unwrap()
    }

    Ok(())
}
