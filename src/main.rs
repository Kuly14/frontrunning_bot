use anyhow::Result;
use mev_bot::config::Config;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::new().await?;
    mev_bot::watch_mempool(config).await?;
    // mev_bot::setup::deploy_mainnet_contract().await;
    Ok(())
}
