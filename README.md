<div align='center'>

<h1>Raydium new pairs listener</h1>
<p>A simple new pairs listener on Raydium written in Rust. </p>

<h4> <span> · </span> <a href="https://github.com/Chall-T/SolWatcher/blob/master/README.md"> Documentation </a> <span> · </span> <a href="https://github.com/Chall-T/SolWatcher/issues"> Report Bug </a> <span> · </span> <a href="https://github.com/Chall-T/SolWatcher/issues"> Request Feature </a> </h4>


</div>


### :key: Environment Variables
`SOL_HTTPS` - Custom Solana https RPC url

`SOL_WSS` - Custom Solana wss RPC url



## :toolbox: Getting Started

### Prerequisites

- Rust<a href="https://www.rust-lang.org/tools/install"> Here</a>


### :running: Run Locally

Clone the project

```bash
https://github.com/Chall-T/SolWatcher.git
```
Go to the project directory
```bash
cd SolWatcher
```
Install dependencies and run
```bash
cargo run
```

### Sample output
```console
[2024-04-20 14:00:53] Setup Solana RPC websocket: "wss://api.mainnet-beta.solana.com"
[2024-04-20 14:00:53] Setup Solana RPC http: "https://api.mainnet-beta.solana.com"
[2024-04-20 14:00:53] Setup Log instruction: "initialize2"
[2024-04-20 14:00:54] Setup Subscribed to Raydium Liquidity Pool
[2024-04-20 14:03:31] Token handler [ERROR] No instructions found
[2024-04-20 14:03:59] Token handler new pair found (Token: 3p8QX1F31JY2JSS3ZmHrmv2gCLR6GW6N9e9VPqUnkQx8 LP Pait: FnwaxPJMHWrZExhBoAoUuC7tNAmqpaBWQSszwLTpnWiq)
[2024-04-20 14:03:59] Token handler new pair found (Token: 3p8QX1F31JY2JSS3ZmHrmv2gCLR6GW6N9e9VPqUnkQx8 LP Pait: FnwaxPJMHWrZExhBoAoUuC7tNAmqpaBWQSszwLTpnWiq)
[2024-04-20 14:04:00] Token handler new pair found (Token: 3p8QX1F31JY2JSS3ZmHrmv2gCLR6GW6N9e9VPqUnkQx8 LP Pait: FnwaxPJMHWrZExhBoAoUuC7tNAmqpaBWQSszwLTpnWiq)
[2024-04-20 14:04:23] Token handler new pair found (Token: NazyMKBTqc2JrVdPZfLt4CZkz2XzbCfdw7o2NQxaD9X LP Pait: 9XKuMdQf4qACvuLmzTaJYc31mfmn4Wp12G137kXAbmUs)
[2024-04-20 14:04:23] Token handler [ERROR] No instructions found
[2024-04-20 14:05:05] Token handler new pair found (Token: 5oDSP4eacy7VSecSEiYZbKbatbSXPvVBu6RZ4Yw1Wjjd LP Pait: 6FkvgvMJz5Au3fuJZfUrrSiM9jXetQVsvodVAiRwtACJ)
[2024-04-20 14:08:26] Token handler [ERROR] No instructions found
[2024-04-20 14:08:26] Token handler new pair found (Token: E8zy2EooUfKeoYQybKxX8aRzFcUGbpbXYxGbb2FnSWs5 LP Pait: EVxPQLVuXWZt7NBy6wQkcfh8ANtj3Dbpqua9bBCH4nY2)
[2024-04-20 14:08:26] Token handler new pair found (Token: E8zy2EooUfKeoYQybKxX8aRzFcUGbpbXYxGbb2FnSWs5 LP Pait: EVxPQLVuXWZt7NBy6wQkcfh8ANtj3Dbpqua9bBCH4nY2)
```
