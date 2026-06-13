# AGENTS.md — condor-eye

## Overview

ISR platform for fixed-wing drones (Fase 0 MVP). Two decoupled subsystems, no implementation yet — repo is a scaffold.

## Project reference docs

- `docs/architecture.md` — data flow, module layout, design principles
- `docs/context.md` — project purpose, domains, pending decisions
- `docs/tech-stack.md` — approved technologies per subsystem
- `docs/roadmap.md` — phase breakdown and milestones

## Subsystems

| Directory | Language | Stack | Responsibility |
|-----------|----------|-------|----------------|
| `ai-python/` | Python 3.12+ | PyTorch, Ultralytics YOLO, OpenCV, NumPy, Pandas, Label Studio, Jupyter | CV pipelines, training, inference, alerts |
| `comms-rust/` | Rust stable | Tokio, Serde, Axum, Tracing | Telemetry, video streaming, gateway, transport |

## Architecture rules

- **No direct coupling between subsystems.** All integration uses abstract interfaces.
- Every module must be replaceable without affecting others.
- Target hardware: Edge (Raspberry Pi, Jetson). Simplicity over optimization during MVP.
- No Kubernetes in Fase 0 — Docker + Docker Compose only.

## Critical pending decision

**ARCH-001** — Communication protocol between subsystems. Evaluate: MAVLink, MQTT, NATS, gRPC, DDS/ROS2, or hybrid. No module may assume a final protocol. See `docs/context.md` and `docs/tech-stack.md`.

## Git conventions

- Conventional Commits (`chore:`, `feat:`, `fix:`, etc.) observed.
- Remote: `git@github.com:KevinFernandez21/condor-eye-backend.git`
- Branch: `main`

## Skill resolution

Agents should consult `.atl/skill-registry.md` when launching subagents. Pass `SKILL.md` paths, not summaries.
