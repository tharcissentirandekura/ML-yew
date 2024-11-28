use mongodb::{options::ClientOptions, Client, Database};

pub async fn connect_db() -> mongodb::error::Result<Database> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    let client = Client::with_options(client_options)?;
    Ok(client.database("ImageDB"))
}
