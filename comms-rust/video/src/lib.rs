// video/ — Captures, buffers, and streams video frames from drone cameras.
//
// Data flow:
//   Drone (RTSP/UDP) -> FrameCapture -> FrameBuffer -> Gateway -> ai-python/inference
//
// TODO: Implement RTSP/UDP video stream ingestion.
// TODO: Add circular frame buffer with configurable capacity (N seconds).
// TODO: Implement frame timestamping and sequence numbering.
// TODO: Add H.264/H.265 decoding support.
// TODO: Implement frame rate limiting for Edge CPU budget.
// TODO: Support thermal camera streams (FLIR) alongside visible spectrum.

use std::time::Duration;

/// A single video frame with metadata.
#[derive(Debug, Clone)]
pub struct VideoFrame {
    /// Monotonic frame sequence number.
    pub sequence: u64,
    /// Capture timestamp.
    pub timestamp_ms: u64,
    /// Raw frame bytes (encoded: H.264, JPEG, etc.).
    pub data: Vec<u8>,
    /// Frame width in pixels.
    pub width: u32,
    /// Frame height in pixels.
    pub height: u32,
    /// Encoding format hint.
    pub encoding: FrameEncoding,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameEncoding {
    H264,
    H265,
    Mjpeg,
    RawRgb,
    Unknown,
}

/// Circular buffer that stores the last N seconds of video frames.
///
/// TODO: Implement lock-free ring buffer with overwrite-eldest semantics.
/// TODO: Support both byte-size and frame-count capacity limits.
/// TODO: Provide iterator for replay (e.g., re-run inference on recent frames).
#[allow(dead_code)]
pub struct FrameBuffer {
    // TODO: Use VecDeque or lock-free ring buffer.
    max_duration: Duration,
}

impl FrameBuffer {
    pub fn new(max_duration: Duration) -> Self {
        FrameBuffer { max_duration }
    }

    /// Push a frame into the buffer. Evicts oldest frames if over capacity.
    pub fn push(&mut self, _frame: VideoFrame) {
        todo!("implement circular push with eviction")
    }

    /// Get the N most recent frames.
    pub fn recent(&self, _count: usize) -> Vec<&VideoFrame> {
        todo!("implement frame retrieval")
    }

    /// Number of frames currently buffered.
    pub fn len(&self) -> usize {
        todo!("return buffer size")
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Captures video frames from a drone stream source.
///
/// TODO: Implement RTSP client with GStreamer or ffmpeg bindings.
/// TODO: Add reconnection logic on stream drop.
/// TODO: Support configurable resolution and frame rate.
#[allow(dead_code)]
pub struct FrameCapture {
    stream_url: String,
}

impl FrameCapture {
    pub fn new(stream_url: String) -> Self {
        FrameCapture { stream_url }
    }

    /// Start capturing frames and feeding them into the provided buffer.
    ///
    /// TODO: Spawn Tokio task that reads from stream and calls buffer.push().
    /// TODO: Handle stream errors gracefully (reconnect with backoff).
    pub async fn start(&self, _buffer: std::sync::Arc<tokio::sync::Mutex<FrameBuffer>>) {
        todo!("implement async frame capture loop")
    }
}

#[cfg(test)]
mod tests {
    // TODO: Add test for circular buffer eviction.
    // TODO: Add test for frame ordering by sequence number.
    // TODO: Add integration test with sample video file.
}
