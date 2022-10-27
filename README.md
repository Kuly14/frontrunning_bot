# MEV Frontrunning Bot written in Rust

This bot is straightforward. It tracks mainnet transactions, if the underlying transaction is a contract call with Input>10 bytes, the bot will try to replicate the transaction on a mainnet fork. If the tx is profitable, it will then run it on mainnet.

I did not test it because I am too cheap to lose some ether on the contract deployment, but if you want, give it a try.
Before you do so, implement flashbots so you will not get front-run frontrunning xD.

# Disclaimer
This is untested software. Only use it if you know what you are doing.

