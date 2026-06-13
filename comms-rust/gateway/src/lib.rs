// gateway/ — Central integration point between subsystems and ground station.
//
// Data flow:
//   telemetry/ -> Gateway -> ai-python/inference (frames + telemetry)
//   video/     -> Gateway -> ai-python/inference (frames)
//   ai-python/ -> Gateway (alerts) -> Ground Control Station
//
// TODO: Implement HTTP/WS API (Axum) for ground station communication.
// TODO: Add channel-based frame/telemetry relay to ai-python.
// TODO: Implement alert forwarding to ground station (WebSocket push).
// TODO: Add health endpoint for subsystem monitoring.
// TODO: Implement subscriber registry for multi-consumer telemetry.

use axum::{Router, routing::get};
use serde::Serialize;

/// Gateway orchestrates data flow between all Condor Eye subsystems.
pub struct Gateway {
    // TODO: Channel senders for telemetry, video frames, alerts.
    // TODO: WebSocket connection registry for ground station clients.
}

#[derive(Debug, Serialize)]
struct HealthStatus {
    telemetry_connected: bool,
    video_stream_active: bool,
    inference_running: bool,
}

impl Gateway {
    pub fn new() -> Self {
        // TODO: Initialize channels and connection pool.
        Gateway {}
    }

    /// Build the Axum router with all API endpoints.
    ///
    /// TODO: Add /api/health, /api/telemetry/stream (SSE), /ws/alerts (WebSocket).
    pub fn router() -> Router {
        Router::new()
            .route("/api/health", get(Self::health_handler))
        // TODO: Add remaining routes.
    }

    async fn health_handler() -> axum::Json<HealthStatus> {
        // TODO: Check actual subsystem status.
        axum::Json(HealthStatus {
            telemetry_connected: false,
            video_stream_active: false,
            inference_running: false,
        })
    }

    /// Start the gateway HTTP server.
    ///
    /// TODO: Bind to configurable host:port.
    /// TODO: Initialize tracing subscriber.
    /// TODO: Graceful shutdown on SIGTERM.
    pub async fn serve(self) {
        let app = Self::router();
        let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
            .await
            .expect("failed to bind");
        tracing_subscriber::fmt::init();
        tracing::info!("Gateway listening on 0.0.0.0:8080");
        axum::serve(listener, app)
            .await
            .expect("gateway server failed");
    }
}

impl Default for Gateway {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    // TODO: Add Axum integration test with test HTTP client.
    // TODO: Add WebSocket connection lifecycle test.
    // TODO: Add telemetry relay throughput benchmark.
}
