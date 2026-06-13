# Condor Eye - Arquitectura

## Fase 0 - MVP

La arquitectura del MVP se compone de dos subsistemas principales desacoplados:

### AI Python (ai-python/)

Módulo responsable del pipeline de visión por computadora:

1. **Datasets** — Recolección, etiquetado y versionado de datasets con Label Studio.
2. **Training** — Entrenamiento de modelos YOLO con PyTorch/Ultralytics.
3. **Inference** — Ejecución de inferencia sobre frames de video en tiempo real.
4. **Evaluation** — Métricas de rendimiento del modelo (mAP, precision, recall).
5. **Notebooks** — Experimentación y prototipado en Jupyter.
6. **Models** — Artefactos de modelos entrenados.

### Communications Rust (comms-rust/)

Módulo responsable de transporte de datos y comunicaciones:

1. **Telemetry** — Recepción y procesamiento de telemetría del dron.
2. **Video** — Streaming y buffer de frames de video.
3. **Gateway** — Punto de integración entre subsistemas y hacia la estación de control.
4. **Integration** — Adaptadores desacoplados para protocolos de comunicación.
5. **Tests** — Pruebas de integración entre subsistemas.

### Flujo de Datos

```
Dron (ArduPilot) → Telemetría → Gateway → AI Inference → Alertas → Estación de Control
                  → Video     → Gateway → AI Inference → Alertas → Estación de Control
```

### Principios de Diseño

- Bajo acoplamiento entre módulos.
- Interfaces abstractas para protocolos de comunicación.
- Todo módulo debe poder reemplazarse sin afectar al resto.
- Compatibilidad con hardware Edge (Raspberry Pi, Jetson).
- Preparado para escalar de 1 dron a N drones.
