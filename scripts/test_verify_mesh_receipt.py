#!/usr/bin/env python3
"""Focused, offline tests for ``verify-mesh-receipt.py``."""

from __future__ import annotations

import hashlib
import importlib.util
import json
import tempfile
import unittest
from pathlib import Path
from typing import Any


SCRIPT = Path(__file__).with_name("verify-mesh-receipt.py")
SPEC = importlib.util.spec_from_file_location("verify_mesh_receipt", SCRIPT)
if SPEC is None or SPEC.loader is None:  # pragma: no cover - import setup failure
    raise RuntimeError(f"unable to load {SCRIPT}")
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


class MeshReceiptVerificationTests(unittest.TestCase):
    def setUp(self) -> None:
        self.tempdir = tempfile.TemporaryDirectory()
        self.root = Path(self.tempdir.name)
        self.manifest = self.root / "manifest.json"
        self.receipt = self.root / "receipt.json"
        self.manifest.write_bytes(b'{"name":"pilot","version":1}\n')
        digest = hashlib.sha256(self.manifest.read_bytes()).hexdigest()
        self.payload: dict[str, Any] = {
            "receipt_id": "receipt-001",
            "input_digest": digest,
            "manifest_sha256": digest,
            "provider": "byteport",
            "execution_backend": "podman",
            "owner": "platform",
            "source": "phenocompose",
            "verified_utc": "2026-08-05T00:00:00Z",
            "evidence": "receipt://receipt-001",
        }

    def tearDown(self) -> None:
        self.tempdir.cleanup()

    def write_receipt(self) -> None:
        self.receipt.write_text(json.dumps(self.payload), encoding="utf-8")

    def write_chain_target(self, receipt_id: str, **fields: str) -> None:
        target = {"receipt_id": receipt_id, **fields}
        (self.root / f"{receipt_id}.json").write_text(
            json.dumps(target), encoding="utf-8"
        )

    def test_valid_receipt_binds_preserved_manifest(self) -> None:
        self.write_receipt()

        valid, reason = MODULE.verify_receipt(self.receipt, self.manifest)

        self.assertTrue(valid)
        self.assertEqual(reason, "valid")

    def test_digest_mismatch_is_stale_and_fail_closed(self) -> None:
        self.write_receipt()
        self.manifest.write_bytes(b'{"name":"replaced"}\n')

        valid, reason = MODULE.verify_receipt(self.receipt, self.manifest)

        self.assertFalse(valid)
        self.assertIn("stale/unverifiable", reason)

    def test_missing_evidence_metadata_is_rejected(self) -> None:
        self.payload.pop("evidence")
        self.write_receipt()

        valid, reason = MODULE.verify_receipt(self.receipt, self.manifest)

        self.assertFalse(valid)
        self.assertIn("evidence", reason)

    def test_receipt_cannot_supersede_itself(self) -> None:
        self.payload["supersedes"] = self.payload["receipt_id"]
        self.write_receipt()

        valid, reason = MODULE.verify_receipt(self.receipt, self.manifest)

        self.assertFalse(valid)
        self.assertIn("cannot supersede itself", reason)

    def test_supersedes_requires_a_reciprocal_preserved_target(self) -> None:
        self.payload["supersedes"] = "receipt-000"
        self.write_receipt()
        self.write_chain_target("receipt-000", superseded_by="receipt-001")

        valid, reason = MODULE.verify_receipt(
            self.receipt, self.manifest, self.root
        )

        self.assertTrue(valid)
        self.assertEqual(reason, "valid")

    def test_supersession_without_target_directory_fails_closed(self) -> None:
        self.payload["supersedes"] = "receipt-000"
        self.write_receipt()

        valid, reason = MODULE.verify_receipt(self.receipt, self.manifest)

        self.assertFalse(valid)
        self.assertIn("chain target directory is required", reason)

    def test_missing_or_nonreciprocal_chain_target_fails_closed(self) -> None:
        self.payload["superseded_by"] = "receipt-002"
        self.write_receipt()
        self.write_chain_target("receipt-002", supersedes="other-receipt")

        valid, reason = MODULE.verify_receipt(
            self.receipt, self.manifest, self.root
        )

        self.assertFalse(valid)
        self.assertIn("must point supersedes back", reason)


if __name__ == "__main__":
    unittest.main()

