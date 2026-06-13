# Condor Eye - Contexto del Proyecto

## ¿Qué es Condor Eye?

Plataforma ISR (Intelligence, Surveillance and Reconnaissance) para drones de ala fija. Enfocada inicialmente en vigilancia de camaroneras, zonas costeras y áreas remotas.

## Estado Actual

**Fase 0 — MVP.** El objetivo es validar la viabilidad técnica de los cuatro pilares:

1. Vuelo autónomo con ArduPilot.
2. Captura de video térmico.
3. Integración de IA para detección de eventos.
4. Comunicación entre subsistemas.

## Dominios

| Dominio | Lenguaje | Responsabilidad |
|---------|----------|-----------------|
| ai-python/ | Python 3.12+ | Visión por computadora, datasets, entrenamiento, inferencia, alertas |
| comms-rust/ | Rust Stable | Telemetría, transporte de datos, video, gateway |

## Principios

- Simplicidad sobre complejidad.
- Modularidad y bajo acoplamiento.
- Capacidad de operar en Edge (Raspberry Pi, Jetson).
- Protocolos de comunicación desacoplados mediante interfaces abstractas.
- Preparado para escalar de 1 dron a N drones.

## Decisión Pendiente

**ARCH-001** — Definir protocolo(s) de comunicación: MAVLink, MQTT, NATS, gRPC, DDS/ROS2, o arquitectura híbrida. Ningún módulo debe asumir un protocolo definitivo hasta que se resuelva.
