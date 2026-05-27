<div align='center'>

<h1>Raydium new pairs listener</h1>
<p>A simple new pairs listener on Raydium (AMM v4 and CPMM) written in Rust.</p>

<h4> <a href="https://github.com/Chall-T/SolWatcher/issues"> Report Bug </a> <span> · </span> <a href="https://github.com/Chall-T/SolWatcher/issues"> Request Feature </a> </h4>


</div>


### :key: Environment Variables

Copy `.env.example` to `.env` and adjust. Main options:

`SOL_HTTPS` / `SOL_WSS` — Solana RPC endpoints (use a dedicated provider for production)

`WATCH_RAYDIUM_AMM` — listen for AMM v4 pools (default: true)

`WATCH_RAYDIUM_CPMM` — listen for CPMM pools (default: false)

`LOG_INSTRUCTION` — log filter for AMM v4 (default: `initialize2`)

`CPMM_LOG_INSTRUCTION` — log filter for CPMM (default: `Instruction: Initialize`)

`RPC_MIN_INTERVAL_MS` — delay between `getTransaction` calls (default: `250`)

`LOG` — set to `debug` for RPC retry messages

See `.env.example` for all variables.


## :toolbox: Getting Started

### Prerequisites

- Rust <a href="https://www.rust-lang.org/tools/install">install</a> (1.89+, see `rust-toolchain.toml`)


### :running: Run Locally

Clone the project

```bash
git clone https://github.com/Chall-T/SolWatcher.git
```

Go to the project directory

```bash
cd SolWatcher
```

Configure environment and run

```bash
cp .env.example .env
cargo run
```

### Sample output

```console
[2026-05-27 23:19:35] Setup Solana RPC websocket: "wss://api.mainnet-beta.solana.com"
[2026-05-27 23:19:35] Setup Solana RPC http: "https://api.mainnet-beta.solana.com"
[2026-05-27 23:19:35] Setup Watcher "AMM v4": program=675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8, log_pattern="initialize2"
[AMM v4] Successfully connected to WebSocket.
[AMM v4] Subscribed to Raydium program 675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8.
[2026-05-27 23:20:01] AMM v4 new pair found (Token: ... LP Pair: ...)
[2026-05-27 23:20:01] CPMM new pair found (Token: ... LP Pair: ...)
```
