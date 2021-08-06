use kanjidic_types::Character;
use mongodb::{options::ClientOptions, Client};
use std::{fs::File, io::BufReader, time::Instant};

type BoxResult = Result<(), Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> BoxResult {
    let now = Instant::now();
    let file = File::open("./assets/kanjidic2.json")?;
    let reader = BufReader::new(file);
    let _characters: Vec<Character> = serde_json::from_reader(reader)?;
    println!("Deserialized: {}", now.elapsed().as_secs());
    let options_string = std::env::var("mongodb_kanjidic_options")?;
    let options = ClientOptions::parse(&options_string).await?;
    println!("Options: {}", now.elapsed().as_secs());
    let client = Client::with_options(options)?;
    println!("Client: {}", now.elapsed().as_secs());
    for db_name in client.list_database_names(None, None).await? {
        println!("{}", db_name);
    }
    Ok(())
}
