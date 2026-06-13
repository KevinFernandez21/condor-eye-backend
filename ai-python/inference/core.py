"""
Real-time inference engine: consumes video frames, produces detections.

Data flow:
    comms-rust/video (frames) -> YOLO inference -> detections -> alerts/
"""

from dataclasses import dataclass, field
from pathlib import Path
from typing import Optional


@dataclass
class Detection:
    """Single detection result with spatial + confidence info."""

    class_id: int
    class_name: str
    confidence: float
    x_center: float
    y_center: float
    width: float
    height: float
    # TODO: Add tracking_id for multi-frame object tracking


@dataclass
class FrameResult:
    """Inference result for a single frame."""

    frame_id: int
    timestamp_ms: int
    detections: list[Detection] = field(default_factory=list)
    # TODO: Add frame metadata (drone_id, gps_coords, altitude)


class InferenceEngine:
    """Runs YOLO inference on video frames, targeting Edge hardware.

    TODO: Implement YOLO model loading with ONNX/TensorRT for Jetson.
    TODO: Add NMS (Non-Maximum Suppression) configuration.
    TODO: Implement frame batching for throughput optimization.
    TODO: Add confidence threshold per class.
    TODO: Support multi-model orchestration (detection + classification).
    """

    def __init__(self, model_path: Path, device: str = "cpu"):
        self._model_path = model_path
        self._device = device
        raise NotImplementedError("InferenceEngine not implemented")

    def load_model(self) -> None:
        """Load model into memory (ONNX Runtime or PyTorch).

        TODO: Detect engine backend (ONNX, TensorRT, PyTorch).
        TODO: Verify model input shape and normalize config.
        """
        raise NotImplementedError

    def infer_frame(self, frame: "bytes") -> FrameResult:
        """Run inference on a single video frame.

        TODO: Decode frame bytes to numpy array (OpenCV).
        TODO: Preprocess (resize, normalize, BGR->RGB).
        TODO: Run forward pass, decode outputs to Detection list.
        TODO: Return structured FrameResult.
        """
        raise NotImplementedError

    def batch_infer(self, frames: list["bytes"]) -> list[FrameResult]:
        """Batch inference for throughput optimization.

        TODO: Stack frames into tensor batch.
        TODO: Measure throughput (FPS) and log.
        """
        raise NotImplementedError

    def shutdown(self) -> None:
        """Release model resources cleanly.

        TODO: Free GPU/CPU memory.
        TODO: Close any open inference sessions.
        """
        raise NotImplementedError
