use anyhow::Result;
use dotenv;
use ethers::{
    prelude::{k256::ecdsa::SigningKey, SignerMiddleware},
    providers::{Http, Middleware, Provider, Ws},
    signers::{LocalWallet, Signer, Wallet},
    types::H160,
};
use std::{str::FromStr, sync::Arc};

#[derive(Debug)]
pub struct Config {
    pub ws: Arc<Provider<Ws>>,
    pub http_domain: String,
    pub http: Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    pub bot_address: H160,
}

impl Config {
    pub async fn new() -> Result<Self> {
        dotenv::dotenv().unwrap();
        let ws_domain = std::env::var("GOERLI_WSS").expect("Failed to get WSS");
        let ws_provider = Provider::<Ws>::connect(ws_domain).await?;
        let ws_provider = Arc::new(ws_provider);

        let http_domain = std::env::var("GOERLI_HTTP").expect("Failed to get HTTP");
        let http_provider = Provider::<Http>::try_from(&http_domain)?;

        let chain_id = ws_provider.get_chainid().await?;

        let private_key = std::env::var("PRIVATE_KEY").expect("Failed to get Private Key");
        let wallet = private_key
            .parse::<LocalWallet>()?
            .with_chain_id(chain_id.as_u64());

        let middleware = SignerMiddleware::new(http_provider, wallet);

        // TODO Add mainnet address here
        let bot_address =
            H160::from_str(&String::from("0x4162a3316fb46e2c9e1cedf459c42f669dcabc3e"))?;

        Ok(Self {
            ws: ws_provider,
            http_domain,
            http: Arc::new(middleware),
            bot_address,
        })
    }
}
