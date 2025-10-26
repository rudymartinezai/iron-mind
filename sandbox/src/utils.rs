use chrono::Local;

use crate::packet::Packet;

pub fn log_startup(config_path: &str) {
    println!(
        "[{}] Iron Mind Sandbox started with config: {}",
        timestamp(),
        config_path
    );
}

pub fn log_firewall_start(rule_count: usize) {
    println!(
        "[{}] Running firewall with {} rules...",
        timestamp(),
        rule_count
    );
}

pub fn log_blocked(packet: &Packet) {
    println!(
        "[{}] Blocked packet from {} -> {} ({} bytes)",
        timestamp(),
        packet.source,
        packet.destination,
        packet.payload_size
    );
}

pub fn log_allowed(packet: &Packet) {
    println!(
        "[{}] Allowed packet from {} -> {} ({} bytes)",
        timestamp(),
        packet.source,
        packet.destination,
        packet.payload_size
    );
}

pub fn log_using_mock_config(path: &str) {
    println!(
        "[{}] Config '{}' missing; falling back to mock rules",
        timestamp(),
        path
    );
}

pub fn default_config() -> &'static str {
    "\
# Mock firewall configuration
allow 10.0.0.
deny 192.168.1.
deny 172.16.
"
}

fn timestamp() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}
