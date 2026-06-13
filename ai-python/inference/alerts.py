"""
Alert generation from inference detections.
Threshold-based rules produce structured alerts for the Gateway.
"""

from dataclasses import dataclass
from enum import Enum
from typing import Optional

from .core import FrameResult


class AlertSeverity(Enum):
    """Alert severity levels."""

    LOW = "low"
    MEDIUM = "medium"
    HIGH = "high"
    CRITICAL = "critical"


@dataclass
class Alert:
    """Structured alert produced from detection events."""

    alert_id: str
    severity: AlertSeverity
    class_detected: str
    confidence: float
    timestamp_ms: int
    frame_id: int
    # TODO: Add GPS coordinates from telemetry (drone position).
    lat: Optional[float] = None
    lon: Optional[float] = None


class AlertGenerator:
    """Converts detection results into actionable alerts.

    TODO: Implement threshold-based alert rules per target class.
    TODO: Add alert deduplication / cooldown (avoid spam).
    TODO: Correlate detections with telemetry GPS coordinates.
    TODO: Define alert format contract with comms-rust/gateway.
    TODO: Implement alert priority queue for burst scenarios.
    """

    def __init__(self, min_confidence: float = 0.5, cooldown_ms: int = 5000):
        self._min_confidence = min_confidence
        self._cooldown_ms = cooldown_ms
        # TODO: Track last-alert timestamp per class for cooldown.

    def process_frame(self, result: FrameResult) -> list[Alert]:
        """Generate alerts from a frame's detections.

        TODO: Filter detections below min_confidence.
        TODO: Map class_ids to human-readable class_names.
        TODO: Apply cooldown per detected class.
        TODO: Enrich with telemetry GPS if available.
        """
        raise NotImplementedError

    def set_telemetry(self, lat: float, lon: float) -> None:
        """Update current drone position from telemetry stream.

        TODO: Called by gateway integration layer before process_frame.
        """
        pass
