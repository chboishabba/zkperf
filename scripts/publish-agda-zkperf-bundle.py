#!/usr/bin/env python3
"""Upload an agda-zkperf bundle to a Hugging Face dataset repo."""

from __future__ import annotations

import argparse
import json
import subprocess
from pathlib import Path
from typing import Any


REQUIRED_FILES = {
    "compact.json",
    "roundtrip.json",
    "stats.json",
    "manifest.json",
}

REQUIRED_ALIASES = [
    ("trace.jsonl", "agda_trace.jsonl"),
    ("source.json", "zkperf_sample_trace.json"),
]


def _load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def main() -> None:
    parser = argparse.ArgumentParser(description="Publish an agda-zkperf bundle to HF as a dataset PR commit.")
    parser.add_argument("bundle_dir", type=Path)
    parser.add_argument("--repo", default="introspector/zkperf")
    parser.add_argument("--path-in-repo", required=True)
    args = parser.parse_args()

    bundle_dir = args.bundle_dir.resolve()
    missing = sorted(name for name in REQUIRED_FILES if not (bundle_dir / name).exists())
    for legacy_name, canonical_name in REQUIRED_ALIASES:
        if not (bundle_dir / legacy_name).exists() and not (bundle_dir / canonical_name).exists():
            missing.append(f"{legacy_name}|{canonical_name}")
    if missing:
        raise SystemExit(f"bundle is missing required files: {', '.join(missing)}")

    manifest = _load_json(bundle_dir / "manifest.json")
    stats = _load_json(bundle_dir / "stats.json")
    if stats.get("roundtrip_equal") is not True:
        raise SystemExit("refusing to publish bundle with non-exact roundtrip")
    commit_message = f"Add agda zkperf bundle for {Path(str(manifest['target'])).name}"
    commit_description = (
        f"agda-delta {manifest['mode']} run for {manifest['target']} "
        f"(run_id={manifest['run_id']}, solved={manifest['solved']}, events={manifest['event_count']}). "
        f"Exact round-trip={stats['roundtrip_equal']} with "
        f"{stats['saved_bytes']} bytes saved ({stats['reduction_ratio']:.2%})."
    )

    proc = subprocess.run(
        [
            "hf",
            "upload",
            args.repo,
            str(bundle_dir),
            args.path_in_repo,
            "--repo-type",
            "dataset",
            "--create-pr",
            "--commit-message",
            commit_message,
            "--commit-description",
            commit_description,
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
                "url": url,
            },
            indent=2,
        )
    )


if __name__ == "__main__":
    main()
