#[cfg(not(feature = "sync"))]
#[cfg_attr(feature = "tokio-runtime", tokio::main)]
#[cfg_attr(feature = "async-std-runtime", async_std::main)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use mongodb::Client;

    let client = Client::with_uri_str("mongodb://localhost:27017").await?;
    let db_count = client.list_databases(None, None).await.unwrap().len();
    println!("database count: {}", db_count);
    Ok(())
}

#[cfg(feature = "sync")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use mongodb::sync::Client;

    let client = Client::with_uri_str("mongodb://localhost:27017")?;
    let db_count = client.list_databases(None, None).unwrap().len();
    println!("database count: {}", db_count);
    Ok(())
}