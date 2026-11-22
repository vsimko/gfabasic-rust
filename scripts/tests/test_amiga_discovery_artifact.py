"""Regression tests for the public Amiga discovery artifact boundary."""

from __future__ import annotations

import base64
import binascii
import pathlib
import struct
import sys
import tempfile
import unittest
import zlib

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
from assert_amiga_discovery_artifact import validate_artifact


def png_chunk(kind: bytes, data: bytes) -> bytes:
    return (struct.pack(">I", len(data)) + kind + data
            + struct.pack(">I", binascii.crc32(kind + data) & 0xFFFFFFFF))


def valid_test_png() -> bytes:
    header = struct.pack(">IIBBBBB", 2, 2, 8, 2, 0, 0, 0)
    pixels = zlib.compress(b"\x00" + b"\xff\x00\x00" * 2
                           + b"\x00" + b"\x00\xff\x00" * 2)
    return (b"\x89PNG\r\n\x1a\n" + png_chunk(b"IHDR", header)
            + png_chunk(b"IDAT", pixels) + png_chunk(b"IEND", b""))


class ArtifactBoundaryTests(unittest.TestCase):
    def setUp(self) -> None:
        self.temp = tempfile.TemporaryDirectory()
        self.root = pathlib.Path(self.temp.name) / "artifact"
        self.root.mkdir()
        (self.root / "archive-info.txt").write_text("archive_size=123\\n", encoding="utf-8")

    def tearDown(self) -> None:
        self.temp.cleanup()

    def test_accepts_approved_utf8_evidence_and_valid_png(self) -> None:
        image_path = self.root / "editor-version-menu.png"
        image_path.write_bytes(valid_test_png())
        self.assertEqual(validate_artifact(self.root), [])

    def test_rejects_unapproved_file_path(self) -> None:
        (self.root / "payload.txt").write_text("diagnostic-looking text", encoding="utf-8")
        self.assertTrue(any("unapproved artifact path: payload.txt" in item
                            for item in validate_artifact(self.root)))

    def test_rejects_utf8_encoded_payload_with_unapproved_extension(self) -> None:
        # Base64/text payloads are valid UTF-8 but are not approved evidence.
        (self.root / "commercial-binary.b64").write_text("TVqQAAMAAAAEAAAA", encoding="utf-8")
        self.assertTrue(any("unapproved artifact path: commercial-binary.b64" in item
                            for item in validate_artifact(self.root)))

    def test_rejects_unapproved_nested_directory(self) -> None:
        nested = self.root / "boot-volume"
        nested.mkdir()
        (nested / "manifest.txt").write_text("metadata", encoding="utf-8")
        violations = validate_artifact(self.root)
        self.assertTrue(any("unapproved directory: boot-volume" in item for item in violations))
        self.assertTrue(any("unapproved artifact path: boot-volume/manifest.txt" in item
                            for item in violations))

    def test_rejects_invalid_png(self) -> None:
        (self.root / "editor-version-menu.png").write_bytes(b"not a png")
        self.assertTrue(any("invalid PNG evidence" in item for item in validate_artifact(self.root)))

    def test_rejects_binary_bytes_in_approved_text_path(self) -> None:
        (self.root / "archive-info.txt").write_bytes(b"metadata\x00payload")
        self.assertTrue(any("NUL byte in text evidence" in item
                            for item in validate_artifact(self.root)))

    def test_rejects_base64_binary_payload_in_approved_text_path(self) -> None:
        payload = base64.b64encode(bytes(range(256)) * 2).decode("ascii")
        (self.root / "archive-info.txt").write_text(
            "archive_size=123\n" + payload + "\n", encoding="utf-8"
        )
        self.assertTrue(any("invalid text format: archive-info.txt" in item
                            for item in validate_artifact(self.root)))

    def test_rejects_base64_payload_in_other_approved_text_path(self) -> None:
        native_run = self.root / "native-run"
        native_run.mkdir()
        payload = base64.b64encode(bytes(range(256)) * 2).decode("ascii")
        (native_run / "result.txt").write_text(
            "probe diagnostic:\n" + payload + "\n", encoding="utf-8"
        )
        self.assertTrue(any("encoded payload in text evidence: native-run/result.txt" in item
                            for item in validate_artifact(self.root)))

    def test_rejects_png_with_ancillary_payload_chunk(self) -> None:
        header = struct.pack(">IIBBBBB", 2, 2, 8, 2, 0, 0, 0)
        payload = base64.b64encode(bytes(range(256)) * 2)
        raster = b"\x00" + b"\xff\x00\x00" * 2 + b"\x00" + b"\x00\xff\x00" * 2
        image = (b"\x89PNG\r\n\x1a\n" + png_chunk(b"IHDR", header)
                 + png_chunk(b"tEXt", b"Comment\x00" + payload)
                 + png_chunk(b"IDAT", zlib.compress(raster))
                 + png_chunk(b"IEND", b""))
        (self.root / "editor-version-menu.png").write_bytes(image)
        self.assertTrue(any("invalid PNG evidence" in item
                            for item in validate_artifact(self.root)))

    def test_rejects_base64url_payload_in_approved_text_path(self) -> None:
        native_run = self.root / "native-run"
        native_run.mkdir()
        payload = base64.urlsafe_b64encode(b"\xff" * 512).decode("ascii").rstrip("=")
        self.assertEqual(set(payload), {"_"})
        (native_run / "result.txt").write_text(payload, encoding="utf-8")
        self.assertTrue(any("encoded payload in text evidence: native-run/result.txt" in item
                            for item in validate_artifact(self.root)))

    def test_rejects_truncated_png_raster(self) -> None:
        header = struct.pack(">IIBBBBB", 2, 2, 8, 2, 0, 0, 0)
        truncated = (b"\x89PNG\r\n\x1a\n" + png_chunk(b"IHDR", header)
                     + png_chunk(b"IDAT", zlib.compress(b""))
                     + png_chunk(b"IEND", b""))
        (self.root / "editor-version-menu.png").write_bytes(truncated)
        self.assertTrue(any("invalid PNG evidence" in item
                            for item in validate_artifact(self.root)))

    def test_rejects_png_with_excessive_raster_expansion(self) -> None:
        header = struct.pack(">IIBBBBB", 1, 1, 8, 2, 0, 0, 0)
        oversized = zlib.compress(b"\x00" * 1_000_000)
        image = (b"\x89PNG\r\n\x1a\n" + png_chunk(b"IHDR", header)
                 + png_chunk(b"IDAT", oversized) + png_chunk(b"IEND", b""))
        (self.root / "editor-version-menu.png").write_bytes(image)
        self.assertTrue(any("invalid PNG evidence" in item
                            for item in validate_artifact(self.root)))

    def test_rejects_control_characters_in_approved_text_path(self) -> None:
        native_run = self.root / "native-run"
        native_run.mkdir()
        (native_run / "result.txt").write_text(
            "diagnostic\x01payload", encoding="utf-8"
        )
        self.assertTrue(any("control character in text evidence: native-run/result.txt" in item
                            for item in validate_artifact(self.root)))


if __name__ == "__main__":
    unittest.main()
