#!/usr/bin/env python3
"""Publish an aggregate HF index for agda zkperf bundles."""

from __future__ import annotations

import argparse
import json
import subprocess
import tempfile
from pathlib import Path
from typing import Any


def _load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def _bundle_entry(bundle_dir: Path) -> dict[str, Any]:
    manifest = _load_json(bundle_dir / "manifest.json")
    stats = _load_json(bundle_dir / "stats.json")
    return {
        "run_id": manifest["run_id"],
        "mode": manifest["mode"],
        "target": manifest["target"],
        "solved": manifest["solved"],
        "event_count": manifest["event_count"],
        "default_hf_path": manifest.get("default_hf_path"),
        "state_machine_schema": manifest.get("state_machine_schema"),
        "roundtrip_equal": stats["roundtrip_equal"],
        "row_count": stats["row_count"],
        "event_type_count": stats["event_count"],
        "saved_bytes": stats["saved_bytes"],
        "reduction_ratio": stats["reduction_ratio"],
    }


def main() -> None:
    parser = argparse.ArgumentParser(description="Publish runs/agda/index.json for agda zkperf bundles.")
    parser.add_argument("bundle_dirs", nargs="+", type=Path)
    parser.add_argument("--repo", default="introspector/zkperf")
    parser.add_argument("--path-in-repo", default="runs/agda/index.json")
    args = parser.parse_args()

    entries = []
    for bundle_dir in args.bundle_dirs:
        bundle_dir = bundle_dir.resolve()
        entry = _bundle_entry(bundle_dir)
        entries.append(entry)

    entries.sort(key=lambda item: str(item["default_hf_path"] or item["target"]))
    payload = {
        "schema_version": "agda-zkperf-run-index/v1",
        "run_count": len(entries),
        "runs": entries,
    }

    with tempfile.TemporaryDirectory(prefix="agda-zkperf-index-") as tmp:
        index_path = Path(tmp) / "index.json"
        index_path.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")
        proc = subprocess.run(
            [
                "hf",
                "upload",
                args.repo,
                str(index_path),
                args.path_in_repo,
                "--repo-type",
                "dataset",
                "--create-pr",
                "--commit-message",
                "Update agda zkperf run index",
                "--commit-description",
                f"Refresh aggregate agda run index with {len(entries)} bundle entries.",
            ],
            check=True,
            capture_output=True,
            text=True,
        )
    url = proc.stdout.strip().splitlines()[-1]
    print(
        json.dumps(
            {
                "repo": args.repo,
                "path_in_repo": args.path_in_repo,
                "run_count": len(entries),
                "url": url,
            },
            indent=2,
        )
    )


if __name__ == "__main__":
    main()
