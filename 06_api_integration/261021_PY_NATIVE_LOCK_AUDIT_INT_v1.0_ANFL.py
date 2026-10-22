#!/usr/bin/env python3
"""The Reactor(TM): strict, public-CI-only native Cargo lock audit.

File: 261021_PY_NATIVE_LOCK_AUDIT_INT_v1.0_ANFL.py
Version: 1.0 | Layer: 06_api_integration | Classification: B
Implementation status: implemented; in-memory self-tests separately reported.
Upstream: openai/codex@83086188ae0614bd882818ff1aa23f3194962012

BLUF: Permit only manifest-proven local 0.0.0 -> 0.163.0 version stamps.
Architecture: pinned original + proposed lock + public workspace Cargo.toml
closure -> strict semantic comparison -> bounded JSON counts/hashes/changes.
No Cargo execution, builds, downloads, writes, configs, secrets, or data files.
Runtime argv: original.lock resolved.lock workspaceRoot, OR --self-test.
The caller must independently bind the public checkout to the upstream commit.
"""

from __future__ import annotations

import copy
import fnmatch
import hashlib
import json
import os
import posixpath
import re
import stat
import sys
import tomllib
import unittest
from contextlib import ExitStack, contextmanager
from functools import lru_cache
from typing import Any, Iterator

ORIGINAL_SHA256 = (
    "17cdd862471774ba7aef836aaf0a0a3ed76bd23e503b1fc7278c89d1f2719d03"
)
RELEASE = "0.163.0"
LOCK_CAP = 4 * 1024 * 1024
MANIFEST_CAP = 256 * 1024
AGGREGATE_CAP = 8 * 1024 * 1024
OUTPUT_CAP = 64 * 1024
MANIFEST_COUNT_CAP = 512
ENTRY_CAP = 20_000
DEPTH_CAP = 32
NAME = re.compile(r"[A-Za-z0-9_-]{1,128}\Z")
VERSION = re.compile(r"[A-Za-z0-9.+-]{1,128}\Z")
Toml = dict[str, Any]


class AuditError(Exception):
    """Represent a fixed, non-sensitive audit refusal code."""

    def __init__(self, code: str) -> None:
        super().__init__(code)
        self.code = code


def require(condition: bool, code: str) -> None:
    """Refuse a failed invariant without formatting untrusted data."""
    if not condition:
        raise AuditError(code)


def digest(raw: bytes) -> str:
    """Return a SHA256 commitment without disclosing input text."""
    return hashlib.sha256(raw).hexdigest()


def directory_identity(value: os.stat_result) -> tuple[int, ...]:
    """Select stable directory metadata, excluding mutable timestamps."""
    return (
        value.st_dev, value.st_ino, value.st_mode, value.st_uid,
        value.st_gid, getattr(value, "st_flags", 0),
    )


def file_identity(value: os.stat_result) -> tuple[int, ...]:
    """Select complete regular-file N9 and flags for a bounded read."""
    return (
        value.st_dev, value.st_ino, value.st_mode, value.st_uid,
        value.st_gid, value.st_nlink, value.st_size, value.st_mtime_ns,
        value.st_ctime_ns, getattr(value, "st_flags", 0),
    )


def relative_path(base: str, value: str) -> str:
    """Normalize a manifest path lexically while refusing root escapes."""
    require(isinstance(value, str) and bool(value), "PATH_SCHEMA")
    require(
        not value.startswith("/") and "\\" not in value
        and all(ord(char) >= 32 and ord(char) != 127 for char in value),
        "PATH_SCHEMA",
    )
    joined = posixpath.normpath(posixpath.join(base, value))
    require(
        joined != ".." and not joined.startswith("../")
        and len(joined.split("/")) <= DEPTH_CAP,
        "PATH_ESCAPE",
    )
    return joined


def glob_matches(path: str, pattern: str) -> bool:
    """Match workspace glob components without allowing star across slash."""
    left = [] if path == "." else path.split("/")
    right = [] if pattern == "." else pattern.split("/")

    @lru_cache(maxsize=None)
    def match(i: int, j: int) -> bool:
        if j == len(right):
            return i == len(left)
        if right[j] == "**":
            return match(i, j + 1) or (
                i < len(left) and match(i + 1, j)
            )
        return (
            i < len(left) and fnmatch.fnmatchcase(left[i], right[j])
            and match(i + 1, j + 1)
        )

    return match(0, 0)


class Reader:
    """Read only no-follow regular files and bounded directory metadata."""

    def __init__(self) -> None:
        self.opened = 0
        self.closed = 0
        self.bytes = 0
        self.entries = 0

    @contextmanager
    def directory(self, name: str, parent: int | None = None) -> Iterator[int]:
        """Hold and close a no-follow directory, checking named identity."""
        fd = os.open(
            name, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW | os.O_CLOEXEC,
            dir_fd=parent,
        )
        self.opened += 1
        before = os.fstat(fd)
        try:
            named = os.stat(name, dir_fd=parent, follow_symlinks=False)
            require(
                stat.S_ISDIR(before.st_mode)
                and directory_identity(before) == directory_identity(named),
                "DIRECTORY_IDENTITY",
            )
            yield fd
            named = os.stat(name, dir_fd=parent, follow_symlinks=False)
            require(
                directory_identity(before) == directory_identity(os.fstat(fd))
                == directory_identity(named),
                "DIRECTORY_UNSTABLE",
            )
        finally:
            os.close(fd)
            self.closed += 1

    @contextmanager
    def absolute_directory(self, path: str) -> Iterator[int]:
        """Descend an explicit directory path without following symlinks."""
        absolute = os.path.abspath(path)
        with ExitStack() as stack:
            fd = stack.enter_context(self.directory("/"))
            for component in absolute.split("/")[1:]:
                if component:
                    fd = stack.enter_context(self.directory(component, fd))
            yield fd

    @contextmanager
    def below(self, root: int, relative: str) -> Iterator[int]:
        """Descend only lexically contained member directories."""
        require(
            relative == relative_path(".", relative), "PATH_SCHEMA"
        )
        with ExitStack() as stack:
            fd = root
            if relative != ".":
                for component in relative.split("/"):
                    fd = stack.enter_context(self.directory(component, fd))
            yield fd

    def read(self, parent: int, leaf: str, cap: int) -> bytes:
        """Read an exact regular-file extent plus EOF and close before parse."""
        require("/" not in leaf and leaf not in ("", ".", ".."), "LEAF_SCHEMA")
        fd = os.open(
            leaf, os.O_RDONLY | os.O_NOFOLLOW | os.O_CLOEXEC | os.O_NONBLOCK,
            dir_fd=parent
        )
        self.opened += 1
        try:
            before = os.fstat(fd)
            named = os.stat(leaf, dir_fd=parent, follow_symlinks=False)
            require(stat.S_ISREG(before.st_mode), "FILE_TYPE")
            require(0 <= before.st_size <= cap, "FILE_CAP")
            require(
                self.bytes + before.st_size <= AGGREGATE_CAP,
                "AGGREGATE_CAP",
            )
            require(
                file_identity(before) == file_identity(named), "FILE_IDENTITY"
            )
            data = bytearray()
            while len(data) < before.st_size:
                block = os.read(fd, min(32768, before.st_size - len(data)))
                require(bool(block), "SHORT_READ")
                data.extend(block)
            require(os.read(fd, 1) == b"", "EOF")
            named = os.stat(leaf, dir_fd=parent, follow_symlinks=False)
            require(
                file_identity(before) == file_identity(os.fstat(fd))
                == file_identity(named), "FILE_UNSTABLE",
            )
            self.bytes += len(data)
            return bytes(data)
        finally:
            os.close(fd)
            self.closed += 1

    def lock(self, path: str) -> bytes:
        """Read one explicitly supplied public lock path."""
        absolute = os.path.abspath(path)
        with self.absolute_directory(os.path.dirname(absolute)) as parent:
            return self.read(parent, os.path.basename(absolute), LOCK_CAP)

    def children(self, parent: int) -> list[str]:
        """Enumerate bounded directory names, never following child links."""
        names: list[str] = []
        with os.scandir(parent) as entries:
            for entry in entries:
                self.entries += 1
                require(self.entries <= ENTRY_CAP, "ENTRY_CAP")
                names.append(entry.name)
        return sorted(names)

    def expand(self, root: int, pattern: str) -> set[str]:
        """Expand bounded workspace member globs using held directories."""
        pattern = relative_path(".", pattern)
        parts = [] if pattern == "." else pattern.split("/")
        result: set[str] = set()

        def visit(relative: str, index: int) -> None:
            require(len(relative.split("/")) <= DEPTH_CAP, "DEPTH_CAP")
            if index == len(parts):
                result.add(relative)
                return
            component = parts[index]
            with self.below(root, relative) as parent:
                if component == "**":
                    visit(relative, index + 1)
                wildcard = any(char in component for char in "*?[")
                names = self.children(parent) if wildcard else [component]
                for name in names:
                    if component != "**" and not fnmatch.fnmatchcase(
                        name, component
                    ):
                        continue
                    try:
                        value = os.stat(
                            name, dir_fd=parent, follow_symlinks=False
                        )
                    except FileNotFoundError:
                        if wildcard:
                            continue
                        raise AuditError("MEMBER_MISSING") from None
                    require(not stat.S_ISLNK(value.st_mode), "MEMBER_SYMLINK")
                    if not stat.S_ISDIR(value.st_mode):
                        require(wildcard, "MEMBER_TYPE")
                        continue
                    child = relative_path(relative, name)
                    visit(child, index if component == "**" else index + 1)

        visit(".", 0)
        return result


def parse(raw: bytes) -> Toml:
    """Decode public TOML without exposing parser diagnostics."""
    try:
        value = tomllib.loads(raw.decode("utf-8", "strict"))
    except (UnicodeError, tomllib.TOMLDecodeError):
        raise AuditError("TOML") from None
    require(isinstance(value, dict), "TOML_SCHEMA")
    return value


def dependency_tables(document: Toml) -> list[Toml]:
    """Select only Cargo dependency descriptor tables."""
    tables: list[Toml] = []
    containers = [document]
    target = document.get("target", {})
    require(isinstance(target, dict), "MANIFEST_SCHEMA")
    containers.extend(target.values())
    for container in containers:
        require(isinstance(container, dict), "MANIFEST_SCHEMA")
        for key in ("dependencies", "dev-dependencies", "build-dependencies"):
            table = container.get(key, {})
            require(isinstance(table, dict), "MANIFEST_SCHEMA")
            tables.append(table)
    return tables


def same_toml(left: Any, right: Any) -> bool:
    """Compare parsed TOML while preserving every scalar/container type."""
    if type(left) is not type(right):
        return False
    if isinstance(left, dict):
        return left.keys() == right.keys() and all(
            same_toml(left[key], right[key]) for key in left
        )
    if isinstance(left, list):
        return len(left) == len(right) and all(
            same_toml(a, b) for a, b in zip(left, right)
        )
    return bool(left == right)


def manifest_version(value: Any, inherited: str) -> str:
    """Resolve only an exact Boolean workspace inheritance declaration."""
    if isinstance(value, dict):
        require(
            value.keys() == {"workspace"} and value["workspace"] is True,
            "VERSION_INHERITANCE",
        )
        return inherited
    require(
        isinstance(value, str) and bool(VERSION.fullmatch(value)),
        "MANIFEST_VERSION",
    )
    return value


def workspace_versions(
    reader: Reader, root: int
) -> tuple[dict[str, str], str, int]:
    """Bind local packages to public workspace/member manifest versions."""
    raw = reader.read(root, "Cargo.toml", MANIFEST_CAP)
    document = parse(raw)
    workspace = document.get("workspace")
    require(isinstance(workspace, dict), "WORKSPACE_SCHEMA")
    package_defaults = workspace.get("package", {})
    require(
        isinstance(package_defaults, dict)
        and package_defaults.get("version") == RELEASE,
        "WORKSPACE_RELEASE",
    )
    inherited = workspace.get("dependencies", {})
    require(isinstance(inherited, dict), "WORKSPACE_DEPENDENCIES")
    members = workspace.get("members", [])
    excluded = workspace.get("exclude", [])
    require(
        isinstance(members, list) and isinstance(excluded, list)
        and all(isinstance(value, str) for value in members + excluded),
        "WORKSPACE_MEMBERS",
    )
    excluded = [relative_path(".", item) for item in excluded]

    def is_excluded(path: str) -> bool:
        return any(glob_matches(path, item) for item in excluded)

    pending: set[str] = {"."} if "package" in document else set()
    for pattern in members:
        pending.update(reader.expand(root, pattern))
    # Cargo can automatically include in-root path dependencies. Reading only
    # their Cargo.toml descriptors also covers test-support packages omitted
    # from the explicit member list. Escapes/nested workspaces are refusals.
    for descriptor in inherited.values():
        if isinstance(descriptor, dict) and "path" in descriptor:
            pending.add(relative_path(".", descriptor["path"]))
    seen: set[str] = set()
    versions: dict[str, str] = {}
    while pending:
        relative = min(pending)
        pending.remove(relative)
        if relative in seen or is_excluded(relative):
            continue
        require(len(seen) < MANIFEST_COUNT_CAP, "MANIFEST_COUNT_CAP")
        seen.add(relative)
        if relative == ".":
            member = document
        else:
            with reader.below(root, relative) as directory:
                member = parse(reader.read(
                    directory, "Cargo.toml", MANIFEST_CAP
                ))
            require("workspace" not in member, "NESTED_WORKSPACE")
        package = member.get("package")
        require(isinstance(package, dict), "PACKAGE_MANIFEST")
        if "workspace" in package:
            require(
                relative_path(relative, package["workspace"]) == ".",
                "OTHER_WORKSPACE",
            )
        name = package.get("name")
        require(isinstance(name, str) and bool(NAME.fullmatch(name)), "NAME")
        version = manifest_version(
            package.get("version"), package_defaults["version"]
        )
        require(name not in versions, "DUPLICATE_MEMBER_NAME")
        versions[name] = version
        for table in dependency_tables(member):
            for dependency, descriptor in table.items():
                base = relative
                if isinstance(descriptor, dict) and descriptor.get("workspace"):
                    require(
                        descriptor.get("workspace") is True,
                        "DEPENDENCY_INHERITANCE",
                    )
                    require(dependency in inherited, "DEPENDENCY_INHERITANCE")
                    descriptor = inherited[dependency]
                    base = "."
                if isinstance(descriptor, dict) and "path" in descriptor:
                    pending.add(relative_path(base, descriptor["path"]))
    return versions, digest(raw), len(seen)


def package_index(
    document: Toml,
) -> tuple[dict[str, Toml], dict[tuple[str, ...], Toml]]:
    """Index unique local and external package identities without normalization."""
    rows = document.get("package")
    require(isinstance(rows, list), "PACKAGE_SCHEMA")
    local: dict[str, Toml] = {}
    external: dict[tuple[str, ...], Toml] = {}
    for row in rows:
        require(isinstance(row, dict), "PACKAGE_SCHEMA")
        name, version = row.get("name"), row.get("version")
        require(
            isinstance(name, str) and bool(NAME.fullmatch(name))
            and isinstance(version, str) and bool(VERSION.fullmatch(version)),
            "PACKAGE_SCHEMA",
        )
        if "source" in row:
            source = row["source"]
            require(isinstance(source, str) and bool(source), "SOURCE_SCHEMA")
            key = (name, version, source)
            require(key not in external, "DUPLICATE_EXTERNAL")
            external[key] = row
        else:
            require(name not in local, "DUPLICATE_LOCAL")
            local[name] = row
    return local, external


def audit(original: Toml, resolved: Toml, versions: dict[str, str]) -> Toml:
    """Allow only exact manifest-backed local release version stamp changes."""
    require(
        same_toml(
            {key: value for key, value in original.items() if key != "package"},
            {key: value for key, value in resolved.items() if key != "package"},
        ),
        "TOP_LEVEL_CHANGED",
    )
    before, external_before = package_index(original)
    after, external_after = package_index(resolved)
    require(same_toml(external_before, external_after), "EXTERNAL_CHANGED")
    require(before.keys() == after.keys(), "LOCAL_ROSTER_CHANGED")
    changes: list[Toml] = []
    for name in sorted(before):
        old, new = before[name], after[name]
        require(name in versions, "LOCAL_NOT_MEMBER")
        require(
            same_toml(
                {key: value for key, value in old.items() if key != "version"},
                {key: value for key, value in new.items() if key != "version"},
            ),
            "LOCAL_FIELDS_CHANGED",
        )
        require(new["version"] == versions[name], "MANIFEST_VERSION_MISMATCH")
        if old["version"] != new["version"]:
            require(
                old["version"] == "0.0.0" and new["version"] == RELEASE,
                "VERSION_TRANSITION",
            )
            changes.append({
                "name": name, "before": old["version"], "after": new["version"]
            })
    return {
        "local_packages": len(before),
        "external_packages": len(external_before),
        "changed_packages": len(changes),
        "changes": changes,
    }


def self_test() -> Toml:
    """Run synthetic in-memory policy cases with no filesystem fixtures."""
    class PolicyTests(unittest.TestCase):
        def setUp(self) -> None:
            self.old = {
                "version": 4, "metadata": {"sentinel": "unchanged"},
                "package": [
                    {"name": "local", "version": "0.0.0",
                     "dependencies": ["serde 1.0.0"]},
                    {"name": "serde", "version": "1.0.0",
                     "source": "registry+synthetic", "checksum": "fixed",
                     "dependencies": ["synthetic-edge"]},
                ],
            }
            self.new = copy.deepcopy(self.old)
            self.new["package"][0]["version"] = RELEASE
            self.versions = {"local": RELEASE}

        def reject(self, code: str) -> None:
            with self.assertRaises(AuditError) as raised:
                audit(self.old, self.new, self.versions)
            self.assertEqual(raised.exception.code, code)

        def test_release_stamp_allowed(self) -> None:
            result = audit(self.old, self.new, self.versions)
            self.assertEqual(result["changed_packages"], 1)

        def test_external_version(self) -> None:
            self.new["package"][1]["version"] = "2.0.0"
            self.reject("EXTERNAL_CHANGED")

        def test_external_checksum(self) -> None:
            self.new["package"][1]["checksum"] = "changed"
            self.reject("EXTERNAL_CHANGED")

        def test_external_dependency(self) -> None:
            self.new["package"][1]["dependencies"] = ["changed"]
            self.reject("EXTERNAL_CHANGED")

        def test_external_extra_field(self) -> None:
            self.new["package"][1]["unexpected"] = True
            self.reject("EXTERNAL_CHANGED")

        def test_external_source(self) -> None:
            self.new["package"][1]["source"] = "git+changed"
            self.reject("EXTERNAL_CHANGED")

        def test_new_external_package(self) -> None:
            self.new["package"].append({
                "name": "new", "version": "1.0.0", "source": "registry+new"
            })
            self.reject("EXTERNAL_CHANGED")

        def test_local_version_must_match_manifest(self) -> None:
            self.new["package"][0]["version"] = "0.164.0"
            self.reject("MANIFEST_VERSION_MISMATCH")

        def test_old_version_transition(self) -> None:
            self.old["package"][0]["version"] = "0.162.0"
            self.reject("VERSION_TRANSITION")

        def test_local_dependency_qualifier_not_normalized(self) -> None:
            self.new["package"][0]["dependencies"] = ["serde"]
            self.reject("LOCAL_FIELDS_CHANGED")

        def test_local_roster(self) -> None:
            self.new["package"].pop(0)
            self.reject("LOCAL_ROSTER_CHANGED")

        def test_top_level_metadata(self) -> None:
            self.new["metadata"]["sentinel"] = "changed"
            self.reject("TOP_LEVEL_CHANGED")

        def test_missing_manifest(self) -> None:
            self.versions.clear()
            self.reject("LOCAL_NOT_MEMBER")

        def test_duplicate_local(self) -> None:
            self.new["package"].append(copy.deepcopy(self.new["package"][0]))
            self.reject("DUPLICATE_LOCAL")

        def test_unchanged_explicit_local_version_allowed(self) -> None:
            self.new["package"][0]["version"] = "0.0.0"
            self.versions["local"] = "0.0.0"
            self.assertEqual(
                audit(self.old, self.new, self.versions)["changed_packages"], 0
            )

        def test_unchanged_stale_version_refused(self) -> None:
            self.new["package"][0]["version"] = "0.0.0"
            self.reject("MANIFEST_VERSION_MISMATCH")

        def test_path_escapes_refused(self) -> None:
            with self.assertRaises(AuditError):
                relative_path(".", "../outside")
            with self.assertRaises(AuditError):
                relative_path("member", "../../outside")


        def test_top_level_integer_float_change(self) -> None:
            self.new["version"] = 4.0
            self.reject("TOP_LEVEL_CHANGED")

        def test_nested_top_level_integer_boolean_change(self) -> None:
            self.old["metadata"]["number"] = [1]
            self.new["metadata"]["number"] = [True]
            self.reject("TOP_LEVEL_CHANGED")

        def test_nested_external_integer_boolean_change(self) -> None:
            self.old["package"][1]["metadata"] = {"number": [1]}
            self.new["package"][1]["metadata"] = {"number": [True]}
            self.reject("EXTERNAL_CHANGED")

        def test_nested_local_integer_float_change(self) -> None:
            self.old["package"][0]["metadata"] = {"number": [1]}
            self.new["package"][0]["metadata"] = {"number": [1.0]}
            self.reject("LOCAL_FIELDS_CHANGED")

        def test_workspace_inheritance_requires_exact_true(self) -> None:
            self.assertEqual(
                manifest_version({"workspace": True}, RELEASE), RELEASE
            )
            for invalid in (1, 1.0, False, "true"):
                with self.assertRaises(AuditError):
                    manifest_version({"workspace": invalid}, RELEASE)

        def test_repeated_recursive_globs_finish_without_backtracking(self) -> None:
            path = "/".join(["segment"] * 24)
            pattern = "/".join(["**"] * 24 + ["absent"])
            self.assertFalse(glob_matches(path, pattern))

        def test_component_globs(self) -> None:
            self.assertTrue(glob_matches("utils/a", "utils/*"))
            self.assertFalse(glob_matches("utils/a/b", "utils/*"))
            self.assertTrue(glob_matches("utils/a/b", "utils/**"))

    result = unittest.TestResult()
    unittest.defaultTestLoader.loadTestsFromTestCase(PolicyTests).run(result)
    return {
        "code": "SELF_TEST_PASS" if result.wasSuccessful() else "SELF_TEST_FAIL",
        "tests": result.testsRun, "failures": len(result.failures),
        "errors": len(result.errors), "filesystem_fixtures": 0,
    }


def main(argv: list[str]) -> int:
    """Audit three explicit public-CI paths or run memory-only self-tests."""
    reader = Reader()
    report: Toml
    status = 2
    try:
        if argv == ["--self-test"]:
            report = self_test()
            status = 0 if report["code"] == "SELF_TEST_PASS" else 2
        else:
            require(len(argv) == 3, "ARGUMENTS")
            original_raw = reader.lock(argv[0])
            require(digest(original_raw) == ORIGINAL_SHA256, "ORIGINAL_SHA256")
            resolved_raw = reader.lock(argv[1])
            with reader.absolute_directory(argv[2]) as root:
                versions, manifest_sha, count = workspace_versions(reader, root)
            require(reader.opened == reader.closed, "FD_CLOSURE")
            result = audit(parse(original_raw), parse(resolved_raw), versions)
            report = {
                "code": "OK", "original_sha256": digest(original_raw),
                "resolved_sha256": digest(resolved_raw),
                "workspace_manifest_sha256": manifest_sha,
                "member_manifests": count, **result,
            }
            status = 0
    except AuditError as error:
        report = {"code": error.code}
    except OSError:
        report = {"code": "FILESYSTEM"}
    except Exception:
        report = {"code": "INTERNAL"}
    report["opened_fds"] = reader.opened
    report["closed_fds"] = reader.closed
    if reader.opened != reader.closed:
        report = {"code": "FD_CLOSURE", "opened_fds": reader.opened,
                  "closed_fds": reader.closed}
        status = 2
    payload = (json.dumps(report, sort_keys=True, separators=(",", ":"))
               + "\n").encode("utf-8")
    if len(payload) > OUTPUT_CAP:
        payload = b'{"code":"OUTPUT_CAP"}\n'
        status = 2
    try:
        sys.stdout.buffer.write(payload)
        sys.stdout.buffer.flush()
    except (OSError, BrokenPipeError):
        return 3
    return status


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
