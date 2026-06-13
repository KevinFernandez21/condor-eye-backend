# Condor Eye - Stack Tecnológico

Este documento define las tecnologías aprobadas para la Fase 0.

## AI Python

Responsabilidades:

* Computer Vision.
* Entrenamiento de modelos.
* Inferencia.
* Procesamiento de imágenes.
* Generación de alertas.

Tecnologías:

* Python 3.12+
* PyTorch
* Ultralytics YOLO
* OpenCV
* NumPy
* Pandas
* Label Studio
* Jupyter Notebook

Estructura:

datasets/
training/
inference/
evaluation/
models/

## Communications Rust

Responsabilidades:

* Telemetría.
* Recepción de eventos.
* Integración con IA.
* Transporte de datos.
* Video streaming.

Tecnologías:

* Rust Stable
* Tokio
* Serde
* Axum (cuando se requiera API)
* Tracing

Estructura:

telemetry/
video/
gateway/
integration/

## Contenedores

Tecnologías aprobadas:

* Docker
* Docker Compose

No utilizar Kubernetes durante la Fase 0.

## Base de Datos

Tecnología recomendada:

* PostgreSQL

No es obligatoria durante las primeras etapas del MVP.

## Frontend

No es prioridad para Fase 0.

Si se requiere una interfaz temporal:

* React
* TypeScript
* Vite

## Protocolos

Pendiente de decisión.

Issue abierta:

ARCH-001

Evaluar:

* MAVLink
* MQTT
* NATS
* gRPC
* DDS / ROS2

Ningún módulo debe asumir un protocolo definitivo.

Toda integración debe realizarse mediante interfaces desacopladas para facilitar cambios futuros.

## Objetivo Técnico

Lograr un MVP funcional capaz de:

* Recibir telemetría.
* Procesar video.
* Ejecutar inferencia.
* Generar alertas.
* Operar sobre hardware Edge.

La validación funcional tiene prioridad sobre la optimización.
