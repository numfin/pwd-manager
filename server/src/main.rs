use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    server::run().await?;
    Ok(())
}
