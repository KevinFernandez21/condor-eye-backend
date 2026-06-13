# Condor Eye - Contexto General del Proyecto

Condor Eye es una plataforma ISR (Intelligence, Surveillance and Reconnaissance) orientada inicialmente a drones de ala fija para vigilancia de camaroneras, zonas costeras y áreas remotas.

El objetivo del MVP es desarrollar un prototipo capaz de:

* Volar de forma autónoma utilizando ArduPilot.
* Capturar video térmico.
* Procesar imágenes mediante modelos de inteligencia artificial.
* Detectar eventos relevantes.
* Generar alertas con coordenadas geográficas.
* Transmitir información hacia una estación de control terrestre.

Actualmente el proyecto se encuentra en Fase 0.

El objetivo principal NO es construir una plataforma comercial completa sino validar:

1. Vuelo autónomo.
2. Captura de video.
3. Integración de IA.
4. Comunicación entre subsistemas.

El proyecto está dividido en dos dominios principales:

1. AI Python
   Responsable de datasets, entrenamiento, inferencia y generación de alertas.

2. Communications Rust
   Responsable de telemetría, transporte de datos, video y futuras integraciones.

Las decisiones sobre protocolos definitivos de comunicación aún no han sido tomadas.

Existe una issue de arquitectura pendiente para definir si el sistema utilizará:

* MAVLink
* MQTT
* NATS
* gRPC
* DDS / ROS2
* Arquitectura híbrida

Por lo tanto cualquier desarrollo debe mantener bajo acoplamiento y permitir reemplazar protocolos en el futuro.

Principios del proyecto:

* Simplicidad sobre complejidad.
* Modularidad.
* Componentes desacoplados.
* Capacidad de operar en Edge.
* Compatibilidad con Raspberry Pi.
* Compatibilidad futura con Jetson.
* Preparación para crecimiento a múltiples sensores IoT.

El repositorio debe mantenerse preparado para evolucionar desde un único dron hacia una plataforma ISR distribuida.
