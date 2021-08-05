use kanjidic_parser::Kanjidic;
use mongodb::{options::ClientOptions, Client};
use std::{fs::File, io::BufReader};

type BoxResult = Result<(), Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> BoxResult {
    let file = File::open("./assets/kanjidic2.json")?;
    let reader = BufReader::new(file);
    let _kanjidic: Kanjidic = serde_json::from_reader(reader)?;
    let options_string = std::env::var("mongodb_kanjidic_options")?;
    let options = ClientOptions::parse(&options_string).await?;
    let client = Client::with_options(options)?;
    for db_name in client.list_database_names(None, None).await? {
        println!("{}", db_name);
    }
    Ok(())
}
