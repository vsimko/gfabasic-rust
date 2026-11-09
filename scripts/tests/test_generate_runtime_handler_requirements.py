from __future__ import annotations

import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from generate_runtime_handler_requirements import GenerationError, generate_source


class RuntimeHandlerRequirementsTests(unittest.TestCase):
    def make_root(self, status: str, handler: str | None, arguments: str) -> Path:
        temp = tempfile.TemporaryDirectory()
        self.addCleanup(temp.cleanup)
        root = Path(temp.name)
        registry = root / "api" / "registry" / "language-core"
        registry.mkdir(parents=True)
        handler_line = "" if handler is None else f'handler = "{handler}"\n'
        (registry / "example.toml").write_text(
            "schema-version = 1\n\n"
            '[[declarations]]\nid = "gfa.language.math.example"\n'
            'name = "EXAMPLE"\nkind = "function"\n'
            '[[declarations.availability]]\ndialect = "example"\nversions = ["1"]\n'
            '[[declarations.syntax]]\nform = "EXAMPLE(...)"\ncontexts = ["expression"]\n'
            f"arguments = {arguments}\n"
            '[declarations.signature]\nreturns = "arithmetic expression"\n'
            '[declarations.implementation]\n'
            f'status = "{status}"\n{handler_line}',
            encoding="utf-8",
        )
        return root

    def test_generates_typed_unary_requirement(self):
        root = self.make_root(
            "partial",
            "gfa.language.math.example",
            '[{ name = "n", type = "integer expression", required = true }]',
        )
        generated = generate_source(root)
        self.assertIn("ImplementationStatus::Partial", generated)
        self.assertIn("HandlerSignature::IntegerToUnsigned", generated)
        self.assertIn('Some("gfa.language.math.example")', generated)

    def test_non_callable_declarations_have_no_handler_signature(self):
        root = self.make_root("not-implemented", None, "[]")
        generated = generate_source(root)
        self.assertIn("ImplementationStatus::NotImplemented", generated)
        self.assertIn("HandlerSignature::Unsupported", generated)
        self.assertIn("handler_id: None", generated)

    def test_callable_declaration_requires_handler_id(self):
        root = self.make_root(
            "implemented",
            None,
            '[{ name = "n", type = "integer expression", required = true }]',
        )
        with self.assertRaisesRegex(GenerationError, "has no handler ID"):
            generate_source(root)

    def test_callable_declaration_rejects_wrong_handler_namespace(self):
        root = self.make_root(
            "partial",
            "gfa.amiga.math.example",
            '[{ name = "n", type = "integer expression", required = true }]',
        )
        with self.assertRaisesRegex(GenerationError, "handler ID namespace"):
            generate_source(root)

    def test_callable_declaration_rejects_unmapped_signature(self):
        root = self.make_root(
            "implemented",
            "gfa.language.math.example",
            '[{ name = "text", type = "string", required = true }]',
        )
        with self.assertRaisesRegex(GenerationError, "unsupported argument signature"):
            generate_source(root)


if __name__ == "__main__":
    unittest.main()
