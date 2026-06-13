"""
Condor Eye AI — Entry point for the Python CV pipeline.

TODO: Wire up the full pipeline: Datasets -> Training -> Inference -> Alerts.
TODO: Add CLI with subcommands (train, infer, evaluate, export).
TODO: Load config from YAML/ENV for model paths, thresholds, device.
TODO: Integrate with comms-rust/gateway for frame and telemetry consumption.
"""


def main():
    # TODO: Parse CLI args (argparse or click).
    # TODO: Dispatch to subcommand: train / infer / evaluate / export-onnx.
    print("Condor Eye AI — Fase 0 MVP")
    print("TODO: implement pipeline orchestration")


if __name__ == "__main__":
    main()
