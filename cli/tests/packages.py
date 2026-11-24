"""Exercise the real CLI and Git offline, including the index trust boundary."""

import os
import sys
import tempfile
import unittest
import subprocess
import concurrent.futures
from pathlib import Path

CLI = Path(sys.argv.pop(1)).resolve()


class Packages(unittest.TestCase):
    def setUp(self):
        temp = tempfile.TemporaryDirectory()
        self.addCleanup(temp.cleanup)
        self.root = Path(temp.name)
        self.project = self.root / "project"
        self.project.mkdir()
        self.manifest = self.project / "vedic.toml"
        self.store = self.root / "user" / "packages"
        self.env = {k: v for k, v in os.environ.items() if not k.startswith("GIT_")}
        self.env.update(
            GIT_CONFIG_GLOBAL=str(self.root / "gitconfig"),
            GIT_CONFIG_NOSYSTEM="1",
            GIT_ALLOW_PROTOCOL="file",
            VEDIC_INSTALL=str(self.store.parent),
        )
        self.index = self.repo("index", "vedic-lang/package-index")
        (self.index / "packages").mkdir()

    def git(self, path, *args):
        return subprocess.check_output(
            ["git", "-C", path, *args], env=self.env, text=True
        ).strip()

    def repo(self, folder, github):
        p = self.root / folder
        p.mkdir()
        self.git(p, "init", "-q", "--initial-branch=main")
        self.git(
            p,
            "config",
            "--global",
            f"url.{p.as_uri()}.insteadOf",
            f"https://github.com/{github}.git",
        )
        return p

    def commit(self, p):
        self.git(p, "add", ".")
        self.git(
            p,
            "-c",
            "user.name=Test",
            "-c",
            "user.email=test@example.invalid",
            "commit",
            "-qm",
            "fixture",
        )
        return self.git(p, "rev-parse", "HEAD")

    def library(self, name, code, deps=""):
        p = self.repo(name, f"example/{name}")
        (p / "vedic.toml").write_text(
            f'[package]\nname = "{name}"\nversion = "1.0.0"\n{deps}', encoding="utf-8"
        )
        (p / "lib.ved").write_text(code, encoding="utf-8")
        return p

    def publish(self, name, revision, version="1.0.0"):
        entry = self.index / "packages" / f"{name}.toml"
        if not entry.exists():
            entry.write_text(f'repository = "example/{name}"\n[releases]\n')
        with entry.open("a") as f:
            f.write(f'"{version}" = "{revision}"\n')
        self.commit(self.index)

    def cli(self, *args, cwd=None, status=0):
        result = subprocess.run(
            [CLI, *args],
            cwd=cwd or self.project,
            env=self.env,
            capture_output=True,
            text=True,
            timeout=30,
        )
        self.assertEqual(result.returncode, status, result)
        return result

    def test_install_import_update_and_restore(self):
        helper = self.library("helper", 'आवाहन "./nested/square.ved";')
        (helper / "nested").mkdir()
        (helper / "nested/square.ved").write_text(
            "सूत्र वर्ग(क) { फल क * क; }", encoding="utf-8"
        )
        self.publish("helper", self.commit(helper))
        sample = self.library(
            "ganita", 'आवाहन "pkg:helper";', '[dependencies]\nhelper = "1.0.0"\n'
        )
        self.publish("ganita", self.commit(sample))
        self.manifest.write_text('# keep this comment\n[project]\nname = "demo"\n')
        nested = self.project / "src"
        nested.mkdir()
        (nested / "main.ved").write_text(
            'आवाहन "pkg:ganita";\nवद(वर्ग(७));', encoding="utf-8"
        )
        self.cli("install", "ganita", cwd=nested)
        text = self.manifest.read_text()
        self.assertIn('# keep this comment\n[project]\nname = "demo"', text)
        self.assertIn('ganita = "1.0.0"', text)
        self.assertNotIn("helper =", text)
        self.assertFalse((nested / "vedic.toml").exists())
        self.assertEqual(self.cli(nested / "main.ved").stdout.strip(), "४९")
        self.cli("install", "ganita")
        self.assertEqual(self.manifest.read_text(), text)

        # An index update cannot change what another checkout restores.
        other = self.root / "second-checkout"
        other.mkdir()
        (other / "vedic.toml").write_text(text)
        (sample / "vedic.toml").write_text(
            '[package]\nname = "ganita"\nversion = "2.0.0"\n'
        )
        self.publish("ganita", self.commit(sample), "2.0.0")
        self.cli("install", cwd=other)
        self.assertFalse((self.store / "ganita/2.0.0").exists())
        self.cli("install", "ganita")
        self.assertTrue((self.store / "ganita/2.0.0/lib.ved").is_file())
        self.assertIn('ganita = "2.0.0"', self.manifest.read_text())
        self.assertEqual((other / "vedic.toml").read_text(), text)

    def test_reject_unlisted_sources_and_incomplete_installs(self):
        broken = self.library("broken", "", '[dependencies]\nunknown = "1.0.0"\n')
        self.publish("broken", self.commit(broken))
        for arg in [
            "example/broken",
            "https://github.com/example/broken",
            "../broken",
            "--index",
        ]:
            self.assertIn(
                "Invalid package name", self.cli("install", arg, status=64).stderr
            )
        self.assertIn(
            "missing from the Vedic index",
            self.cli("install", "other", status=1).stderr,
        )
        self.assertIn("unknown", self.cli("install", "broken", status=1).stderr)
        self.assertFalse(self.manifest.exists())
        self.assertFalse((self.store / "broken/1.0.0").exists())

        self.manifest.write_text("[dependencies\n")
        self.cli("install", "broken", status=1)
        self.assertEqual(self.manifest.read_text(), "[dependencies\n")

    def test_validate_index_and_source_before_recording_a_release(self):
        lib = self.library("ganita", "")
        sha = self.commit(lib)
        self.publish("ganita", sha)
        entry = self.index / "packages/ganita.toml"
        original = entry.read_text()
        for invalid, message in [
            (
                original.replace("example/ganita", "https://example.org/arbitrary.git"),
                "Invalid repository",
            ),
            (original.replace(sha, "main"), "full lowercase Git commit"),
            (original.replace("1.0.0", "1.0.1"), "manifest does not match"),
        ]:
            entry.write_text(invalid)
            self.commit(self.index)
            self.assertIn(message, self.cli("install", "ganita", status=1).stderr)
            self.assertFalse(self.manifest.exists())

        # Commit a symlink using the Git index so the test also runs on Windows.
        blob = self.git(lib, "rev-parse", "HEAD:lib.ved")
        self.git(lib, "update-index", "--add", "--cacheinfo", f"120000,{blob},link.ved")
        self.git(
            lib,
            "-c",
            "user.name=Test",
            "-c",
            "user.email=test@example.invalid",
            "commit",
            "-qm",
            "symlink",
        )
        entry.write_text(original.replace(sha, self.git(lib, "rev-parse", "HEAD")))
        self.commit(self.index)
        self.assertIn(
            "regular source file", self.cli("install", "ganita", status=1).stderr
        )
        self.assertFalse(self.manifest.exists())

    def test_dependency_cycle_is_reported_without_committing_partial_state(self):
        a = self.library("alpha", "", '[dependencies]\nbeta = "1.0.0"\n')
        b = self.library("beta", "", '[dependencies]\nalpha = "1.0.0"\n')
        self.publish("alpha", self.commit(a))
        self.publish("beta", self.commit(b))
        self.assertIn("Circular", self.cli("install", "alpha", status=1).stderr)
        self.assertFalse(self.manifest.exists())
        self.assertFalse((self.store / "alpha/1.0.0").exists())

    def test_concurrent_adds_preserve_both_dependencies(self):
        names = ["alpha", "beta"]
        for name in names:
            self.publish(name, self.commit(self.library(name, "")))
        with concurrent.futures.ThreadPoolExecutor() as pool:
            list(pool.map(lambda n: self.cli("install", n), names))
        for name in names:
            self.assertIn(f'{name} = "1.0.0"', self.manifest.read_text())


if __name__ == "__main__":
    unittest.main()
