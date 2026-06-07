#!/usr/bin/env python3
"""Emit a deterministic sample-trace artifact bundle for zkperf."""

from __future__ import annotations

import argparse
import importlib.util
import json
import shutil
import unittest
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[1]
CODEC_PATH = ROOT / "scripts" / "compact-sample-trace.py"
TEST_PATH = ROOT / "scripts" / "test-compact-sample-trace.py"


def _load_module(name: str, path: Path):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    assert spec is not None and spec.loader is not None
    spec.loader.exec_module(module)
    return module


def _write_json(path: Path, payload: dict[str, Any]) -> None:
    path.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")


def _relative_stats(
    codec,
    source_path: Path,
    compact_path: Path,
    roundtrip_path: Path,
) -> dict[str, Any]:
    stats = codec.compression_stats(source_path, compact_path, roundtrip_path)
    return {
        "source": source_path.name,
        "compact": compact_path.name,
        "roundtrip": roundtrip_path.name,
        "source_bytes": stats["source_bytes"],
        "compact_bytes": stats["compact_bytes"],
        "saved_bytes": stats["saved_bytes"],
        "reduction_ratio": stats["reduction_ratio"],
        "row_count": stats["row_count"],
        "event_count": stats["event_count"],
        "roundtrip_equal": stats["roundtrip_equal"],
    }


def _run_regression_tests(test_module) -> dict[str, Any]:
    suite = unittest.defaultTestLoader.loadTestsFromModule(test_module)
    result = unittest.TestResult()
    suite.run(result)
    return {
        "successful": result.wasSuccessful(),
        "tests_run": result.testsRun,
        "failures": len(result.failures),
        "errors": len(result.errors),
        "skipped": len(getattr(result, "skipped", [])),
        "expected_failures": len(getattr(result, "expectedFailures", [])),
        "unexpected_successes": len(getattr(result, "unexpectedSuccesses", [])),
    }


def build_bundle(output_dir: Path) -> dict[str, Any]:
    codec = _load_module("compact_sample_trace", CODEC_PATH)
    tests = _load_module("compact_sample_trace_tests", TEST_PATH)

    if output_dir.exists():
        shutil.rmtree(output_dir)
    output_dir.mkdir(parents=True)

    source_payload = codec.decode_trace(tests.FIXTURE_COMPACT)
    source_path = output_dir / "source.json"
    compact_path = output_dir / "compact.json"
    roundtrip_path = output_dir / "roundtrip.json"
    stats_path = output_dir / "stats.json"
    tests_path = output_dir / "tests.json"

    _write_json(source_path, source_payload)
    stats_payload = _relative_stats(codec, source_path, compact_path, roundtrip_path)
    tests_payload = _run_regression_tests(tests)
    _write_json(stats_path, stats_payload)
    _write_json(tests_path, tests_payload)

    return {
        "bundle_dir": str(output_dir),
        "files": [
            source_path.name,
            compact_path.name,
            roundtrip_path.name,
            stats_path.name,
            tests_path.name,
        ],
        "stats": stats_payload,
        "tests": tests_payload,
    }


def main() -> None:
    parser = argparse.ArgumentParser(description="Emit a deterministic sample-trace artifact bundle.")
    parser.add_argument(
        "--output-dir",
        type=Path,
        default=ROOT / "out" / "sample-trace-edition",
        help="Directory to populate with source, compact, roundtrip, stats, and test outputs.",
    )
    args = parser.parse_args()
    print(json.dumps(build_bundle(args.output_dir), indent=2))


if __name__ == "__main__":
    main()
