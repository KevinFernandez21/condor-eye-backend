// integration/ — Protocol adapters for communication between subsystems.
//
// Every adapter implements abstract traits so the protocol can be swapped
// without touching business logic. See ARCH-001 for the pending decision.
//
// TODO: Define CommunicationProtocol trait (send_telemetry, send_alert, ping).
// TODO: Implement MQTT adapter (Paho MQTT client).
// TODO: Implement MAVLink adapter (direct serial).
// TODO: Implement NATS adapter (async-nats).
// TODO: Implement gRPC adapter (tonic).
// TODO: Add protocol benchmarking harness (latency, throughput, reliability).
// TODO: Implement protocol auto-detection / configuration.

/// Abstract communication protocol trait.
///
/// Every subsystem integration MUST go through this trait.
/// No module may import a concrete protocol implementation directly.
///
/// TODO: Finalize trait methods based on ARCH-001 evaluation.
/// TODO: Ensure trait is object-safe for dynamic dispatch.
pub trait CommunicationProtocol: Send + Sync {
    /// Send a telemetry packet to the AI subsystem.
    fn send_telemetry(
        &self,
        _sys_id: u8,
        _lat: f64,
        _lon: f64,
        _alt: f64,
    ) -> Result<(), ProtocolError>;

    /// Forward an alert to the ground control station.
    fn send_alert(&self, _alert_json: &str) -> Result<(), ProtocolError>;

    /// Health check — ping the other side.
    fn ping(&self) -> Result<u64, ProtocolError>;

    /// Human-readable protocol name for logging.
    fn protocol_name(&self) -> &'static str;
}

#[derive(Debug)]
pub struct ProtocolError {
    pub message: String,
    // TODO: Add error kind (Timeout, ConnectionLost, Serialization, etc.).
}

impl std::fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ProtocolError: {}", self.message)
    }
}

impl std::error::Error for ProtocolError {}

/// Selects and instantiates the configured protocol adapter.
///
/// TODO: Read configuration from env vars or config file.
/// TODO: Validate that chosen protocol satisfies project requirements.
pub fn create_protocol(_protocol_name: &str) -> Box<dyn CommunicationProtocol> {
    todo!("implement protocol factory based on ARCH-001 decision")
}

#[cfg(test)]
mod tests {
    // TODO: Add mock protocol adapter for unit testing.
    // TODO: Add serialization roundtrip tests for each protocol.
    // TODO: Add failure injection tests (timeout, disconnect, malformed data).
}
