#[cfg(feature = "v111")]
use mongodb_111 as mongodb;
#[cfg(feature = "v111-tokio")]
use tokio_111 as tokio;

use mongodb::{Client, options::ClientOptions};

#[cfg(feature = "v111-tokio")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    let client = Client::with_options(client_options)?;
    let db_count = client.list_databases(None, None).await.unwrap().len();
    println!("database count: {}", db_count);
/* 
    for db_name in client.list_database_names(None, None).await? {
        println!("{}", db_name);
    }
*/
    Ok(())
}
