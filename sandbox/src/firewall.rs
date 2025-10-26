use crate::packet::Packet;
use crate::utils;

pub fn run(rules: Vec<String>) {
    utils::log_firewall_start(rules.len());
    let packets = Packet::mock_packets();

    for packet in packets {
        if rules.iter().any(|rule| packet.source.contains(rule)) {
            utils::log_blocked(&packet);
        } else {
            utils::log_allowed(&packet);
        }
    }
}
