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
import re
import sys
from pathlib import Path
from typing import Any


REQUIRED_FIELDS = (
    "receipt_id",
    "input_digest",
    "input_digest_kind",
    "manifest_sha256",
    "provider",
    "execution_backend",
    "owner",
    "source",
    "verified_utc",
    "evidence",
)

# ``input_digest`` and ``manifest_sha256`` bind the preserved artifact bytes.
# PhenoCompose's plan digest is a different value: it is computed from the
# canonicalized manifest representation before the artifact is preserved.
RAW_MANIFEST_DIGEST_KIND = "manifest_bytes_sha256"
COMPOSITION_DIGEST_KIND = "phenocompose_manifest_v0_canonical_json"
COMPOSITION_DIGEST_RE = re.compile(r"^sha256:[0-9a-f]{64}$")


def _fail(message: str) -> tuple[bool, str]:
    return False, message


def verify_receipt(receipt_path: Path, manifest_path: Path) -> tuple[bool, str]:
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
    if receipt["input_digest_kind"] != RAW_MANIFEST_DIGEST_KIND:
        return _fail(
            "ambiguous input_digest: input_digest_kind must be "
            f"{RAW_MANIFEST_DIGEST_KIND}"
        )
    if input_digest != digest or manifest_digest != digest:
        return _fail(
            "stale/unverifiable: preserved manifest digest does not match "
            "input_digest and manifest_sha256"
        )

    composition_digest = receipt.get("composition_digest")
    composition_digest_kind = receipt.get("composition_digest_kind")
    if composition_digest is not None or composition_digest_kind is not None:
        if composition_digest_kind != COMPOSITION_DIGEST_KIND:
            return _fail(
                "ambiguous composition_digest: composition_digest_kind must be "
                f"{COMPOSITION_DIGEST_KIND}"
            )
        if not isinstance(composition_digest, str) or not COMPOSITION_DIGEST_RE.fullmatch(
            composition_digest
        ):
            return _fail(
                "invalid composition_digest: expected sha256:<64 lowercase hex>"
            )
    if receipt.get("receipt_id") == receipt.get("supersedes"):
        return _fail("receipt_id cannot supersede itself")
    if receipt.get("superseded_by") == receipt.get("receipt_id"):
        return _fail("receipt_id cannot be superseded by itself")
    return True, "valid"


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("receipt", type=Path)
    parser.add_argument("manifest", type=Path)
    args = parser.parse_args(argv)
    valid, reason = verify_receipt(args.receipt, args.manifest)
    print(("VALID" if valid else "INVALID") + ": " + reason)
    return 0 if valid else 1


if __name__ == "__main__":
    sys.exit(main())
