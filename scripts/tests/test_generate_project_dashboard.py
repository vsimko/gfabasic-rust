import importlib.util
import tempfile
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
GENERATOR_PATH = ROOT / "scripts" / "generate_project_data.py"
SPEC = importlib.util.spec_from_file_location("generate_project_data", GENERATOR_PATH)
GENERATOR = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(GENERATOR)


class StaticDashboardTests(unittest.TestCase):
    def sample_data(self):
        return {
            "source_revision": "0123456789abcdef",
            "project": {
                "maturity": "experimental",
                "target": "Linux x86-64",
                "source_dialect": "Amiga GFA BASIC 3.5",
            },
            "workspace_packages": [{"name": "compiler", "path": "crates/compiler"}],
            "api_progress": {"implemented": 0, "total": 36, "percent": 0, "population": "Confirmed 3.5 addendum declarations only."},
            "roadmap_progress": [
                {
                    "number": 1,
                    "name": "Foundation",
                    "description": "Establish the project foundation.",
                    "completed": 2,
                    "total": 4,
                    "percent": 50,
                    "status": "in-progress",
                    "issue_links": [{"label": "Foundation task", "url": "https://github.com/vsimko/gfabasic-rust/issues/303"}],
                },
                {
                    "number": 2,
                    "name": "Runtime",
                    "description": "Build the runtime.",
                    "completed": 0,
                    "total": 3,
                    "percent": 0,
                    "status": "planned",
                    "issue_links": [],
                },
            ],
            "roadmap_blockers": [{
                "summary": "Runtime depends on the foundation.",
                "issue_links": [{"label": "Dependency", "url": "https://github.com/vsimko/gfabasic-rust/issues/304"}],
            }],
            "components": [{
                "id": "frontend",
                "name": "Compiler frontend",
                "status": "prototype",
                "rationale": "A narrow compiler prototype exists.",
                "evidence": ["README.md#experimental-prototype"],
                "related_issues": [303],
            }],
        }

    def test_generated_page_contains_complete_static_dashboard(self):
        data = self.sample_data()
        dashboard, roadmap = GENERATOR.render_project_dashboard(data)

        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            (root / "site").mkdir()
            (root / "site" / "index.html").write_text(
                '<main><section id="dashboard"><div class="dashboard-content">'
                '<!-- GENERATED DASHBOARD START --><!-- GENERATED DASHBOARD END -->'
                '</div></section><section id="roadmap"><div class="roadmap-list">'
                '<!-- GENERATED ROADMAP START --><!-- GENERATED ROADMAP END -->'
                '</div></section></main>',
                encoding="utf-8",
            )
            GENERATOR.render_static_site(root, data)
            page = (root / "site" / "index.html").read_text(encoding="utf-8")

        self.assertIn('id="dashboard"', page)
        self.assertIn("Project maturity", page)
        self.assertIn("experimental", page)
        self.assertIn("Linux x86-64", page)
        self.assertIn("Workspace", page)
        self.assertIn("Implemented 3.5 addendum API", page)
        self.assertIn("0 / 28", page)
        self.assertIn("not the complete Amiga GFA BASIC 3.5 language API", page)
        self.assertIn("Roadmap progress", page)
        self.assertIn("Phase 1 · Foundation", page)
        self.assertIn("2/4 · 50%", page)
        self.assertIn('style="width:50%"', page)
        self.assertIn("Implementation states", page)
        self.assertIn("Compiler frontend", page)
        self.assertIn("README.md#experimental-prototype", page)
        self.assertIn("issues/303", page)
        self.assertIn("Dependencies &amp; sequencing rules", page)
        self.assertIn("issues/304", page)
        self.assertNotIn("Loading generated repository data", page)
        self.assertNotIn("Loading canonical roadmap phases", page)
        self.assertNotIn("Loading…", page)


    def test_addendum_progress_counts_only_fully_implemented_35_declarations(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            registry = root / "api" / "registry" / "language-core"
            registry.mkdir(parents=True)
            (registry / "sample.toml").write_text(
                """
[[declarations]]
id = "included-implemented"
[declarations.evidence]
source-scope = "amiga-3.5-addendum"
[declarations.implementation]
status = "implemented"
[[declarations.availability]]
dialect = "amiga-gfa-basic-interpreter"
versions = ["3.5"]

[[declarations]]
id = "included-prototype"
[declarations.evidence]
source-scope = "amiga-3.5-addendum"
[declarations.implementation]
status = "prototype"
[[declarations.availability]]
dialect = "amiga-gfa-basic-interpreter"
versions = ["3.5"]

[[declarations]]
id = "excluded-base-manual"
[declarations.evidence]
source-scope = "amiga-3.0-base-manual"
[declarations.implementation]
status = "implemented"
[[declarations.availability]]
dialect = "amiga-gfa-basic-interpreter"
versions = ["3.5"]

[[declarations]]
id = "excluded-other-version"
[declarations.evidence]
source-scope = "amiga-3.5-addendum"
[declarations.implementation]
status = "implemented"
[[declarations.availability]]
dialect = "amiga-gfa-basic-interpreter"
versions = ["3.0"]
""",
                encoding="utf-8",
            )
            result = GENERATOR.load_addendum_api_progress(root)

        self.assertEqual(result["implemented"], 1)
        self.assertEqual(result["total"], 2)
        self.assertEqual(result["percent"], 50)

    def test_repository_addendum_population_is_explicit_and_scoped(self):
        result = GENERATOR.load_addendum_api_progress(ROOT)
        self.assertEqual(result["total"], 28)
        self.assertEqual(result["implemented"], 0)

    def test_dashboard_renderer_escapes_repository_content(self):
        data = self.sample_data()
        data["components"][0]["name"] = '<script>alert("x")</script>'
        dashboard, _ = GENERATOR.render_project_dashboard(data)
        self.assertNotIn('<script>alert("x")</script>', dashboard)
        self.assertIn("&lt;script&gt;", dashboard)


if __name__ == "__main__":
    unittest.main()
