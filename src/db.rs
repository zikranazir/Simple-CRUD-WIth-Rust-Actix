use mongodb::{Client, options::ClientOptions, Database};
use std::error::Error;
use dotenv::dotenv;
use std::env;

pub async fn connect() -> Result<Database, Box<dyn Error>> {
    dotenv().ok(); // Memuat variabel dari file .env

    let database_url = env::var("MONGODB_URI")?; // Mendapatkan URI dari file .env
    let client_options = ClientOptions::parse(database_url).await?;
    let client = Client::with_options(client_options)?;

    Ok(client.database("db_derau")) // Ganti dengan nama database yang sesuai
}