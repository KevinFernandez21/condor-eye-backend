// gateway/src/main.rs — Entry point for the Condor Eye gateway server.
//
// TODO: Read config from env (BIND_ADDR, LOG_LEVEL, PROTOCOL).
// TODO: Initialize telemetry listener before serving HTTP.
// TODO: Wire up video frame capture pipeline.
// TODO: Connect to ai-python inference service over chosen protocol.

use gateway::Gateway;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("Condor Eye Gateway — Fase 0 MVP");

    // TODO: Initialize all subsystem connections before serving.
    let gateway = Gateway::new();
    gateway.serve().await;
}
