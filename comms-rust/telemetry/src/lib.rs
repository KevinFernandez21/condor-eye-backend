// telemetry/ — Receives and processes drone telemetry data.
//
// Data flow:
//   Drone (ArduPilot) -> MAVLink/Serial -> TelemetryParser -> Gateway
//
// TODO: Implement MAVLink v2 message parsing for ArduPilot.
// TODO: Add telemetry validation (checksum, sequence gaps, heartbeat timeout).
// TODO: Implement GPS coordinate extraction (GLOBAL_POSITION_INT).
// TODO: Add attitude and heading extraction (ATTITUDE).
// TODO: Implement battery/voltage monitoring (SYS_STATUS).
// TODO: Support multiple concurrent drone connections.
// TODO: Buffer telemetry stream for batch processing and replay.

use serde::{Deserialize, Serialize};

/// Structured telemetry packet consumed by the gateway.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryPacket {
    /// Drone system ID (MAVLink sysid).
    pub sys_id: u8,
    /// Timestamp in milliseconds since epoch.
    pub timestamp_ms: u64,
    /// Latitude in degrees * 1e7 (MAVLink convention).
    pub lat: i32,
    /// Longitude in degrees * 1e7.
    pub lon: i32,
    /// Altitude MSL in millimeters.
    pub alt_msl: i32,
    /// Ground speed in cm/s.
    pub ground_speed: u16,
    /// Heading in centidegrees (0..36000).
    pub heading: u16,
    /// Battery remaining as percentage (0..100).
    pub battery_remaining: u8,
}

/// Parses raw MAVLink byte streams into structured telemetry.
pub struct TelemetryParser {
    // TODO: Store per-drone state (last heartbeat, sequence counters).
}

impl TelemetryParser {
    pub fn new() -> Self {
        // TODO: Initialize parser state.
        TelemetryParser {}
    }

    /// Feed raw bytes into the parser. Returns complete packets when available.
    ///
    /// TODO: Buffer partial messages across calls.
    /// TODO: Validate MAVLink CRC.
    /// TODO: Emit warnings on sequence gaps or heartbeat timeouts.
    pub fn feed(&mut self, _raw: &[u8]) -> Vec<TelemetryPacket> {
        todo!("implement MAVLink v2 message parsing")
    }
}

impl Default for TelemetryParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Add test with real MAVLink byte samples.
    // TODO: Add test for partial message buffering.
    // TODO: Add test for CRC validation failure.
    // TODO: Add test for heartbeat timeout detection.
}
