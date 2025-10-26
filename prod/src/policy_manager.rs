use crate::blockchain_logger::LedgerSync;
use log::info;

pub async fn sync_policies() {
    info!("Starting policy sync placeholder");

    let mut ledger = LedgerSync::new();
    ledger.queue("policy snapshot");
    ledger.flush().await;

    info!("Policy sync placeholder finished");
}
