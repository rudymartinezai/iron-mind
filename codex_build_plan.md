# 🧠 Codex Build Plan: Iron Mind

This file instructs Codex (or any AI code generator) on how to scaffold and build the **Iron Mind** Rust firewall project.

## 📁 Project Structure

```
iron-mind/
│
├── sandbox/                # Local-only prototype (fully functional)
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       ├── firewall.rs
│       ├── packet.rs
│       ├── config.rs
│       └── utils.rs
│
├── prod/                   # Production build with blockchain integration placeholders
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       ├── firewall_core.rs
│       ├── packet_inspector.rs
│       ├── blockchain_logger.rs
│       ├── node_sync.rs
│       ├── policy_manager.rs
│       ├── telemetry.rs
│       └── lib.rs
│
├── README.md
├── ROADMAP.md
└── codex_build_plan.md
```

---

## ⚙️ Phase 1: Sandbox Prototype

### Description
A minimal, local Rust firewall simulation that can:
- Parse a mock configuration file.
- Simulate incoming packets.
- Filter packets based on basic allow/deny rules.
- Print logs to stdout.

### TODO for Codex
- [ ] Generate `sandbox/Cargo.toml` with dependencies:
  ```toml
  [package]
  name = "iron_mind_sandbox"
  version = "0.1.0"
  edition = "2021"

  [dependencies]
  clap = { version = "4.4", features = ["derive"] }
  chrono = "0.4"
  ```

- [ ] Implement `sandbox/src/main.rs`:
  ```rust
  mod firewall;
  mod packet;
  mod config;
  mod utils;

  use clap::Parser;

  #[derive(Parser)]
  struct Args {
      #[arg(short, long, default_value = "config.toml")]
      config: String,
  }

  fn main() {
      let args = Args::parse();
      println!("Iron Mind Sandbox started with config: {}", args.config);

      let rules = config::load_rules(&args.config);
      firewall::run(rules);
  }
  ```

- [ ] Implement `sandbox/src/firewall.rs`:
  ```rust
  use crate::packet::Packet;

  pub fn run(rules: Vec<String>) {
      println!("Running firewall with {} rules...", rules.len());
      let packets = vec![Packet::new("10.0.0.1"), Packet::new("192.168.1.5")];

      for p in packets {
          if rules.iter().any(|r| p.source.contains(r)) {
              println!("Blocked packet from {}", p.source);
          } else {
              println!("Allowed packet from {}", p.source);
          }
      }
  }
  ```

- [ ] Implement placeholder files (`packet.rs`, `config.rs`, `utils.rs`) with basic structs and mock data.

- [ ] Confirm `cargo run` compiles and runs cleanly on macOS.

---

## 🧩 Phase 2: Production Scaffolding (Blockchain-Ready)

### Description
A production-level architecture ready for blockchain integration (Substrate, Tendermint, or custom). Actual blockchain code is **NOT** implemented, but placeholder modules are prepared.

### TODO for Codex
- [ ] Create `prod/Cargo.toml` with expanded dependencies:
  ```toml
  [package]
  name = "iron_mind"
  version = "0.1.0"
  edition = "2021"

  [dependencies]
  clap = { version = "4.4", features = ["derive"] }
  tokio = { version = "1", features = ["full"] }
  serde = { version = "1.0", features = ["derive"] }
  toml = "0.8"
  log = "0.4"
  env_logger = "0.10"
  # Placeholder blockchain crates (to be filled later)
  # substrate-api-client = "..."  
  ```

- [ ] Implement `prod/src/main.rs`:
  ```rust
  mod firewall_core;
  mod blockchain_logger;
  mod policy_manager;
  mod telemetry;

  #[tokio::main]
  async fn main() {
      env_logger::init();
      log::info!("Iron Mind Production starting...");

      firewall_core::init().await;
      policy_manager::sync_policies().await;
      telemetry::start().await;
  }
  ```

- [ ] Create stub modules with TODO markers like:
  ```rust
  // TODO: Implement blockchain-backed event logging
  pub async fn log_event(event: &str) {
      println!("Blockchain placeholder: {}", event);
  }
  ```

- [ ] Create placeholder structs for rule validation and ledger syncing.

---

## 🧰 macOS Setup Instructions

1. Install Rust:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. Restart terminal, then run:
   ```bash
   rustc --version
   cargo --version
   ```
3. Clone the repo and enter sandbox mode:
   ```bash
   git clone https://github.com/YOURNAME/iron-mind.git
   cd iron-mind/sandbox
   cargo run
   ```
4. Once sandbox compiles, move to production folder for scaffolding.

---

## ✅ Output Goal

When Codex finishes, running:
```bash
cd sandbox
cargo run
```
should produce something like:
```
Iron Mind Sandbox started with config: config.toml
Running firewall with 2 rules...
Allowed packet from 10.0.0.1
Blocked packet from 192.168.1.5
```
