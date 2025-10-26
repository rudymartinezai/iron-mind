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
