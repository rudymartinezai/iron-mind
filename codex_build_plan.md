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

## 🧩 Phase 2: Production Scaffolding (Blockchain-Ready)

### Description
A production-level architecture ready for blockchain integration (Substrate, Tendermint, or custom). Actual blockchain code is **NOT** implemented, but placeholder modules are prepared.

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
