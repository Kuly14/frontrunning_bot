use anyhow::Result;
use ethers::{
    prelude::k256::ecdsa::SigningKey,
    prelude::*,
    providers::{Middleware, Provider, StreamExt, TransactionStream},
    utils::{Anvil, AnvilInstance},
};
use std::sync::Arc;
use std::time::Duration;

pub mod config;
pub mod setup;

abigen!(Bot, "artifacts/abi/bot.json");

pub async fn watch_mempool(config: config::Config) -> Result<()> {
    let tx_hash_stream = config.ws.subscribe_pending_txs().await?;
    let mut tx_stream = TransactionStream::new(&config.ws, tx_hash_stream, 256);

    while let Some(maybe_tx) = tx_stream.next().await {
        if let Ok(tx) = maybe_tx {
            if let Some(tx_to) = tx.to {
                if tx.input.len() > 10 {
                    let success = match try_tx(&config, &tx_to, &tx.input, &tx.gas).await {
                        Ok(b) => b,
                        Err(e) => {
                            eprintln!("{}", e);
                            false
                        }
                    };

                    if success {
                        execute_trade(&config, &tx_to, &tx.input, &tx.gas).await;
                    }
                }
            }
        }
    }
    Ok(())
}

pub async fn execute_trade(config: &config::Config, to: &H160, input: &Bytes, gas: &U256) {
    let bot = Bot::new(config.bot_address, config.http.clone());
    match bot.frontrun_bytes(*to, input.clone(), *gas).send().await {
        Ok(receipt) => match receipt.await {
            Ok(_) => println!("Trade on mainnet successful!!!"),
            Err(e) => eprintln!("Failed mainnnet trade with this error: {}", e),
        },
        Err(e) => eprintln!("Failed with this error: {}", e),
    };
}

pub async fn try_tx(config: &config::Config, to: &H160, input: &Bytes, gas: &U256) -> Result<bool> {
    let block_num = config.http.get_block_number().await?;
    let anvil = Anvil::new()
        .fork(config.http_domain.clone())
        .fork_block_number(block_num.as_u64())
        .spawn();
    let provider = get_anvil_provider(&anvil)?;

    let bot = deploy_contract_fork(&provider).await?;

    match bot.frontrun_bytes(*to, input.clone(), *gas).send().await {
        Ok(_) => {
            println!("Transaction successful on mainnet-fork, moving onto real chain");
            return Ok(true);
        }
        Err(e) => {
            eprintln!("Failed with: {}", e);
            return Ok(false);
        }
    };
}

pub async fn deploy_contract_fork(
    provider: &SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
) -> Result<bot::Bot<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>> {
    let bot_contract = Bot::deploy(Arc::new(provider.clone()), ())?.send().await?;
    Ok(bot_contract)
}

pub fn get_anvil_provider(
    anvil: &AnvilInstance,
) -> Result<Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>> {
    dotenv::dotenv().unwrap();
    let wallet: LocalWallet = std::env::var("PRIVATE_KEY")?.parse::<LocalWallet>()?;
    let provider =
        Provider::<Http>::try_from(anvil.endpoint())?.interval(Duration::from_millis(10u64));

    let client = SignerMiddleware::new(provider, wallet.with_chain_id(anvil.chain_id()));

    Ok(Arc::new(client))
}
