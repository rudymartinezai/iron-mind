use crate::blockchain_logger;
use log::info;

pub struct RuleValidator {
    rule_count: usize,
}

impl RuleValidator {
    pub fn new(rule_count: usize) -> Self {
        Self { rule_count }
    }

    pub fn validate(&self, rule: &str) -> bool {
        !rule.trim().is_empty()
    }

    pub fn rule_count(&self) -> usize {
        self.rule_count
    }
}

pub async fn init() {
    let validator = RuleValidator::new(0);
    if validator.validate("allow all") {
        info!("Firewall validator primed for {} rules", validator.rule_count());
    }

    blockchain_logger::log_event("firewall initialized").await;
}
