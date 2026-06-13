# Condor Eye - Roadmap

## Fase 0 — MVP (Actual)

**Objetivo:** Validar la integración de vuelo autónomo, captura de video, IA y comunicaciones.

- [ ] Estructura del repositorio.
- [ ] Definición de protocolos de comunicación (ARCH-001).
- [ ] Recepción de telemetría básica (comms-rust/telemetry).
- [ ] Captura y buffer de video (comms-rust/video).
- [ ] Pipeline de inferencia inicial (ai-python/inference).
- [ ] Generación de alertas con coordenadas (ai-python/inference + comms-rust/gateway).
- [ ] Integración end-to-end: dron → telemetría → video → IA → alerta.
- [ ] Contenedores Docker para todos los servicios.

## Fase 1 — Estabilización

- [ ] Modelo entrenado con datasets reales de camaroneras.
- [ ] Evaluación de rendimiento en hardware Edge.
- [ ] Interfaz de monitoreo básica (Frontend opcional).
- [ ] Pruebas de campo con dron real.

## Fase 2 — Escalamiento

- [ ] Soporte multi-dron.
- [ ] Sensores IoT adicionales.
- [ ] Plataforma distribuida.
- [ ] Dashboards y analíticas.
