use std::{fs::File, io::BufReader};
use kanjidic_parser::Kanjidic;
use mongodb::{Client, options::{self, ClientOptions, ServerAddress}};
use tokio::task::spawn_blocking;

type BoxResult = Result<(), Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> BoxResult {
    let file = File::open("./assets/kanjidic2.json")?;
    let reader = BufReader::new(file);
    // let kanjidic: Kanjidic = serde_json::from_reader(reader)?;
    let options_string = std::env::var("mongodb_kanjidic_options")?;
    // let options_string = "Stuff";
    println!("{:?}", options_string);
    let options = ClientOptions::parse(&options_string).await?;
    println!("{:?}", options);
    let client = Client::with_options(options)?;
    println!("{:?}", client);
    for db_name in client.list_database_names(None, None).await? {
        println!("{}", db_name);
    }
    Ok(())
}
