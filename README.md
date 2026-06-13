# Condor Eye

Plataforma ISR (Intelligence, Surveillance and Reconnaissance) para drones de ala fija.
Enfocada en vigilancia de camaroneras, zonas costeras y areas remotas.

**Fase 0 — MVP.** Objetivo: validar la integracion de vuelo autonomo, captura de video termico, IA y comunicaciones.

## Inicio rapido

```bash
git clone git@github.com:KevinFernandez21/condor-eye-backend.git
cd condor-eye

# AI Python (requiere Python 3.12+)
cd ai-python
python -m venv .venv && source .venv/bin/activate
pip install -e ".[dev]"

# Comms Rust (requiere Rust stable)
cd ../comms-rust
cargo build
```

## Estructura

```
condor-eye/
├── ai-python/        # Computer Vision, entrenamiento, inferencia, alertas
│   ├── datasets/     # Recoleccion y etiquetado con Label Studio
│   ├── training/     # Entrenamiento YOLO con Ultralytics + PyTorch
│   ├── inference/    # Inferencia en tiempo real + generacion de alertas
│   ├── evaluation/   # Metricas de rendimiento (mAP, precision, recall)
│   ├── models/       # Artefactos de modelos entrenados
│   └── notebooks/    # Experimentacion Jupyter
├── comms-rust/       # Transporte de datos y comunicaciones
│   ├── telemetry/    # Recepcion y procesamiento de telemetria
│   ├── video/        # Streaming y buffer de frames
│   ├── gateway/      # Punto de integracion y API HTTP/WS
│   └── integration/  # Adaptadores para protocolos de comunicacion
├── docker/           # Docker Compose para desarrollo y despliegue Edge
└── docs/             # Documentacion de arquitectura y decisiones
```

## Stack

| Subsistema | Lenguaje | Tecnologias |
|------------|----------|-------------|
| `ai-python/` | Python 3.12+ | PyTorch, Ultralytics YOLO, OpenCV, NumPy, Pandas, Label Studio |
| `comms-rust/` | Rust stable | Tokio, Serde, Axum, Tracing |
| Contenedores | — | Docker, Docker Compose (sin Kubernetes en Fase 0) |
| Base de datos | — | PostgreSQL (opcional durante MVP) |

## Decisiones clave

| ID | Estado | Descripcion |
|----|--------|-------------|
| ARCH-001 | Pendiente | Protocolo de comunicacion entre subsistemas: MAVLink, MQTT, NATS, gRPC, DDS/ROS2 o hibrido. Ningun modulo asume un protocolo definitivo. |

## Reglas de arquitectura

- Bajo acoplamiento entre subsistemas. Toda integracion usa interfaces abstractas.
- Cada modulo debe poder reemplazarse sin afectar al resto.
- Target hardware: Edge (Raspberry Pi, Jetson). Simplicidad sobre optimizacion.
- Sin Kubernetes en Fase 0.

## Comandos

```bash
# Python: formatear, tipar, testear
ruff check ai-python/
ruff format ai-python/
mypy ai-python/
pytest ai-python/tests/

# Rust: compilar, formatear, testear
cargo build --manifest-path comms-rust/Cargo.toml
cargo fmt --manifest-path comms-rust/Cargo.toml -- --check
cargo clippy --manifest-path comms-rust/Cargo.toml
cargo test --manifest-path comms-rust/Cargo.toml
```

## Recursos

- `docs/architecture.md` — Flujo de datos, modulos, principios de diseno
- `docs/context.md` — Proposito del proyecto, dominios, decisiones pendientes
- `docs/tech-stack.md` — Tecnologias aprobadas por subsistema
- `docs/roadmap.md` — Fases y milestones
- `AGENTS.md` — Instrucciones para sesiones OpenCode
