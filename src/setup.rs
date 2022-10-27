use ethers::prelude::*;
use ethers_solc::Solc;
use std::fs::File;
use std::io::prelude::*;

use crate::{config, Bot};

// Unecessary we can just use abigen! from ethers
pub fn setup() {
    let (abi, bytecode) = {
        let contracts = Solc::default().compile_source("contracts/Bot.sol").unwrap();
        let abi = &contracts
            .get("contracts/Bot.sol", "Bot")
            .unwrap()
            .abi
            .unwrap();
        let bytecode = contracts
            .get("contracts/Bot.sol", "Bot")
            .unwrap()
            .bytecode()
            .unwrap()
            .clone();
        (serde_json::to_string(abi).unwrap(), bytecode)
    };

    let mut file_abi = File::create("artifacts/abi/bot.json").unwrap();
    file_abi.write_all(&abi.as_bytes()).unwrap();

    let mut file_bytecode = File::create("artifacts/abi/bytecode.txt").unwrap();
    file_bytecode.write_all(&bytecode).unwrap();

    let bindings = Abigen::new("Bot", abi).unwrap().generate().unwrap();
    bindings.write_to_file("artifacts/bindings/Bot.rs").unwrap();
}

pub async fn deploy_mainnet_contract() -> () {
    let config = config::Config::new().await.unwrap();
    let contract = Bot::deploy(config.http, ()).unwrap().send().await.unwrap();
    println!("{:#?}", contract.address());
}
