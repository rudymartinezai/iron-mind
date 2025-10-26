#[derive(Debug, Clone)]
pub struct Packet {
    pub source: String,
    pub destination: String,
    pub payload_size: usize,
}

impl Packet {
    pub fn new<S: Into<String>>(source: S) -> Self {
        Packet::with_details(source, "172.16.0.1", 512)
    }

    pub fn with_details<S: Into<String>, D: Into<String>>(
        source: S,
        destination: D,
        payload_size: usize,
    ) -> Self {
        Self {
            source: source.into(),
            destination: destination.into(),
            payload_size,
        }
    }

    pub fn mock_packets() -> Vec<Packet> {
        vec![
            Packet::new("10.0.0.1"),
            Packet::new("192.168.1.5"),
            Packet::with_details("8.8.8.8", "172.16.0.30", 64),
        ]
    }
}
