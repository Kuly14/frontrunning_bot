# MEV Frontrunning Bot written in Rust

This bot is very simple. It tracks mainnet transactions if they have some data so if it's some kind of contract call, the bot will launch a mainnet-fork, it will try to replicate the tx there and if it's profitable it will run the transaction on mainnet with higher gas than the intiail transaction.

I didn't test it on mainnet since I'm too cheap to loose some ether on the contract deployment, but if you want give it a try.

Before you do so definitelly implement flashbots so you won't get frontrun frontrunning xD. 


# Disclaimer

This is untested software only use it if you know what you are doing.

