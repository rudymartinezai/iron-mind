use log::{debug, info};

pub struct LedgerSync {
    pending: Vec<String>,
}

impl LedgerSync {
    pub fn new() -> Self {
        Self { pending: Vec::new() }
    }

    pub fn queue(&mut self, entry: impl Into<String>) {
        self.pending.push(entry.into());
    }

    pub async fn flush(&mut self) {
        if self.pending.is_empty() {
            debug!("Ledger sync flush with no pending entries");
            return;
        }

        info!("Flushing {} ledger entries to blockchain placeholder", self.pending.len());
        self.pending.clear();
    }
}

pub async fn log_event(event: &str) {
    info!("Blockchain placeholder: {}", event);
}
