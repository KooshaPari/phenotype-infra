#!/usr/bin/env python3
"""Fail-closed verification for a preserved compute-mesh receipt.

The verifier is deliberately filesystem-only: it never contacts a provider or
changes the receipt.  A receipt is valid only if the preserved manifest bytes,
receipt identity fields, and evidence metadata are all present and consistent.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import sys
from pathlib import Path
from typing import Any


REQUIRED_FIELDS = (
    "receipt_id",
    "input_digest",
    "manifest_sha256",
    "provider",
    "execution_backend",
    "owner",
    "source",
    "verified_utc",
    "evidence",
)


def _fail(message: str) -> tuple[bool, str]:
    return False, message


def _chain_target(
    receipts_dir: Path,
    target_id: Any,
    receipt_id: str,
    reciprocal_field: str,
) -> tuple[bool, str]:
    """Verify one receipt-ID link without following provider or network state."""
    if not isinstance(target_id, str) or not target_id.strip():
        return _fail("chain target must be a non-empty receipt ID")
    if Path(target_id).name != target_id or target_id in {".", ".."}:
        return _fail("chain target must be a single receipt ID, not a path")

    root = receipts_dir.resolve()
    target_path = (root / f"{target_id}.json").resolve()
    if target_path.parent != root:
        return _fail("chain target resolves outside the receipt directory")
    try:
        target: dict[str, Any] = json.loads(target_path.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError) as exc:
        return _fail(f"chain target {target_id!r} unreadable: {exc}")
    if not isinstance(target, dict):
        return _fail(f"chain target {target_id!r} must be a JSON object")
    if target.get("receipt_id") != target_id:
        return _fail(f"chain target {target_id!r} has a mismatched receipt_id")
    if target.get(reciprocal_field) != receipt_id:
        return _fail(
            f"chain target {target_id!r} must point {reciprocal_field} "
            f"back to {receipt_id!r}"
        )
    return True, "valid"


def _verify_chain(receipt: dict[str, Any], receipts_dir: Path | None) -> tuple[bool, str]:
    """Require verifiable, reciprocal links for optional supersession fields."""
    supersedes = receipt.get("supersedes")
    superseded_by = receipt.get("superseded_by")
    if supersedes is None and superseded_by is None:
        return True, "valid"
    if receipts_dir is None:
        return _fail(
            "chain target directory is required to verify supersedes/superseded_by"
        )
    receipt_id = receipt["receipt_id"]
    if supersedes is not None:
        valid, reason = _chain_target(
            receipts_dir, supersedes, receipt_id, "superseded_by"
        )
        if not valid:
            return valid, reason
    if superseded_by is not None:
        valid, reason = _chain_target(
            receipts_dir, superseded_by, receipt_id, "supersedes"
        )
        if not valid:
            return valid, reason
    return True, "valid"


def verify_receipt(
    receipt_path: Path,
    manifest_path: Path,
    receipts_dir: Path | None = None,
) -> tuple[bool, str]:
    """Return ``(valid, reason)`` without mutating either input file."""
    try:
        receipt: dict[str, Any] = json.loads(receipt_path.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError) as exc:
        return _fail(f"receipt unreadable: {exc}")
    if not isinstance(receipt, dict):
        return _fail("receipt must be a JSON object")

    missing = [field for field in REQUIRED_FIELDS if not receipt.get(field)]
    if missing:
        return _fail("missing required fields: " + ", ".join(missing))

    try:
        manifest_bytes = manifest_path.read_bytes()
    except OSError as exc:
        return _fail(f"preserved manifest unreadable: {exc}")
    digest = hashlib.sha256(manifest_bytes).hexdigest()
    input_digest = receipt["input_digest"]
    manifest_digest = receipt["manifest_sha256"]
    if input_digest != digest or manifest_digest != digest:
        return _fail(
            "stale/unverifiable: preserved manifest digest does not match "
            "input_digest and manifest_sha256"
        )
    if receipt.get("receipt_id") == receipt.get("supersedes"):
        return _fail("receipt_id cannot supersede itself")
    if receipt.get("superseded_by") == receipt.get("receipt_id"):
        return _fail("receipt_id cannot be superseded by itself")
    return _verify_chain(receipt, receipts_dir)


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("receipt", type=Path)
    parser.add_argument("manifest", type=Path)
    parser.add_argument(
        "--receipt-dir",
        type=Path,
        help="directory containing immutable <receipt_id>.json chain targets",
    )
    args = parser.parse_args(argv)
    valid, reason = verify_receipt(args.receipt, args.manifest, args.receipt_dir)
    print(("VALID" if valid else "INVALID") + ": " + reason)
    return 0 if valid else 1


if __name__ == "__main__":
    sys.exit(main())

