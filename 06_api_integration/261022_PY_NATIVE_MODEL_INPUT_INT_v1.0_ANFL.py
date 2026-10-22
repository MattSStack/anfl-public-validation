#!/usr/bin/env python3
"""Compare released-native synthetic model inputs through a loopback receiver.

ANFL | The Reactor™ | Layer 06 | Version 1.0 | 2026-10-22
Implementation status: planned; execution evidence is recorded separately.
Synthetic turns write their own disposable chats. The receiver executes no model.
"""

from __future__ import annotations

import argparse
import hashlib
import http.server
import importlib.util
import json
import os
import platform
import subprocess
import sys
import tempfile
import threading
import time
import urllib.request
from dataclasses import dataclass, field
from pathlib import Path
from types import ModuleType
from typing import Any

PROBE_SHA = "f0e3d7a5bde0e09e4953c23474bc5039c03a2aeb0d5f0db3c1fe9d2dae64ad1f"
PREFIX = "ANFL_PUBLIC_OLDER_PREFIX_MUST_BE_DISCARDED"
TAIL = "ANFL_PUBLIC_MODEL_TAIL_MUST_BE_RETAINED"
REQUEST = "ANFL_PUBLIC_CAPTURE_ONLY_DO_NOT_EXECUTE_TOOLS"
MAX_BODY = 1_048_576
ERROR_BODY = json.dumps({
    "error": {"message": "ANFL_PUBLIC_FIXED_CAPTURE_ERROR",
              "type": "invalid_request_error", "code": "public_capture_only"}
}).encode()


def require(condition: bool, code: str) -> None:
    """Fail the experiment explicitly without interpreting partial success."""
    if not condition:
        raise RuntimeError(code)


def emit(event: str, **fields: Any) -> None:
    """Emit public-only bounded structured evidence."""
    print(json.dumps({"event": event, **fields}, sort_keys=True), flush=True)


def unique_object(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    """Reject duplicate JSON keys instead of silently replacing their values."""
    result: dict[str, Any] = {}
    for key, value in pairs:
        require(key not in result, "DUPLICATE_JSON_KEY")
        result[key] = value
    return result


def canonical(value: Any) -> bytes:
    """Canonicalize object keys while retaining all ordered values and fields."""
    return json.dumps(value, sort_keys=True, separators=(",", ":")).encode()


def cases(base: bytes) -> dict[str, bytes]:
    """Derive independent full, bounded and missing-checkpoint baselines."""
    rows = [json.loads(line) for line in base.splitlines()]
    require(len(rows) == 5 and rows[1]["type"] == "compacted",
            "QUALIFIED_FIXTURE_CONTRACT")

    def message(text: str) -> dict[str, Any]:
        return {
            "timestamp": rows[0]["timestamp"],
            "type": "response_item",
            "payload": {
                "type": "message", "role": "user",
                "id": "msg_public_prefix" if text == PREFIX else "msg_public_tail",
                "content": [{"type": "input_text", "text": text}],
            },
        }

    rows[1]["payload"]["replacement_history"][0]["id"] = (
        "msg_public_replacement"
    )
    combinations = {
        "full": [rows[0], message(PREFIX), rows[1], message(TAIL), *rows[2:]],
        "bounded": [rows[0], rows[1], message(TAIL), *rows[2:]],
        "negative": [rows[0], message(PREFIX), message(TAIL), *rows[2:]],
    }
    result: dict[str, bytes] = {}
    for name, sequence in combinations.items():
        independent = json.loads(json.dumps(sequence))
        for ordinal, row in enumerate(independent):
            row["ordinal"] = ordinal
        result[name] = b"".join(canonical(row) + b"\n" for row in independent)
    return result


def normalized_input(
    values: list[dict[str, Any]],
) -> tuple[bytes, list[dict[str, Any]]]:
    """Normalize only unreferenced fresh instruction/current-user message IDs.

    Fixed historical fixture IDs and every other field remain unchanged.
    No call IDs, references, content, roles, order or opaque values are removed.
    This function is specific to these controlled public synthetic fixtures.
    """
    raw = canonical(values)
    copied = json.loads(raw)
    known = {"msg_public_prefix", "msg_public_tail", "msg_public_replacement"}
    changes: list[dict[str, Any]] = []
    for index, item in enumerate(copied):
        if item.get("type") != "message" or "id" not in item:
            continue
        identity = item["id"]
        if identity in known:
            continue
        require(isinstance(identity, str) and 0 < len(identity) <= 256,
                "VOLATILE_MESSAGE_ID_TYPE")
        content = canonical(item.get("content", []))
        role = item.get("role")
        allowed = role in ("developer", "system") or (
            role == "user" and REQUEST.encode() in content
            and PREFIX.encode() not in content and TAIL.encode() not in content
        )
        require(allowed, "UNDECLARED_MESSAGE_ID_NORMALIZATION")
        require(raw.count(canonical(identity)) == 1, "MESSAGE_ID_IS_REFERENCED")
        item["id"] = f"ANFL_PUBLIC_VOLATILE_MESSAGE_ID_AT_{index}"
        changes.append({"index": index, "field": "id",
                        "role": role, "raw_id": identity})
    return canonical(copied), changes


@dataclass
class Capture:
    """Own the finite receiver state for exactly one synthetic native turn."""

    name: str
    done: threading.Event = field(default_factory=threading.Event)
    calls: int = 0
    payload: dict[str, Any] | None = None
    error: str | None = None
    fixed_response_written: bool = False


class Receiver(http.server.HTTPServer):
    """Bind only IPv4 loopback and serve a single active synthetic case."""

    active: Capture | None = None
    get_calls: int = 0

    def get_request(self) -> tuple[Any, Any]:
        """Bound every accepted connection's socket operations."""
        stream, address = super().get_request()
        stream.settimeout(5)
        return stream, address


class Handler(http.server.BaseHTTPRequestHandler):
    """Capture bounded JSON and always return a deterministic non-retry error."""

    def log_message(self, format: str, *args: Any) -> None:
        """Suppress unrestricted default HTTP access logging."""

    def reply(self, code: int, body: bytes) -> None:
        """Send a finite response and close this owned HTTP connection."""
        self.send_response(code)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(body)))
        self.send_header("Connection", "close")
        self.end_headers()
        self.wfile.write(body)
        self.close_connection = True

    def do_GET(self) -> None:
        """Provide only the exact positive network-canary endpoint."""
        server: Receiver = self.server  # type: ignore[assignment]
        server.get_calls += 1
        if self.path == "/canary":
            self.reply(200, b'{"public_canary":true}')
        else:
            if server.active is not None:
                server.active.error = "UNEXPECTED_GET"
            self.reply(404, b"{}")

    def do_POST(self) -> None:
        """Read one bounded request without storing credentials or raw headers."""
        server: Receiver = self.server  # type: ignore[assignment]
        state = server.active
        try:
            require(state is not None, "NO_ACTIVE_CASE")
            state.calls += 1
            require(state.calls == 1, "DUPLICATE_NATIVE_REQUEST")
            require(self.path == "/v1/responses", "UNEXPECTED_PROVIDER_PATH")
            require(self.headers.get("Transfer-Encoding") is None,
                    "CHUNKED_INPUT_UNSUPPORTED")
            require(self.headers.get("Content-Encoding") in (None, "identity"),
                    "ENCODED_INPUT_UNSUPPORTED")
            require(self.headers.get("Authorization") is None,
                    "UNEXPECTED_AUTHORIZATION_HEADER")
            length = self.headers.get("Content-Length", "")
            require(length.isdecimal() and len(length) <= 8,
                    "CONTENT_LENGTH_REQUIRED")
            size = int(length)
            require(0 < size <= MAX_BODY, "REQUEST_BODY_CAP")
            raw = self.rfile.read(size)
            require(len(raw) == size, "REQUEST_BODY_TRUNCATED")
            payload = json.loads(raw, object_pairs_hook=unique_object)
            require(isinstance(payload, dict), "REQUEST_OBJECT_REQUIRED")
            require(isinstance(payload.get("input"), list), "MODEL_INPUT_LIST")
            require(all(isinstance(item, dict) for item in payload["input"]),
                    "MODEL_INPUT_ITEMS")
            state.payload = payload
        except Exception as error:
            if state is not None:
                state.error = f"{type(error).__name__}:{error}"
        finally:
            try:
                self.reply(400, ERROR_BODY)
                if state is not None:
                    state.fixed_response_written = True
            except Exception as error:
                if state is not None:
                    state.error = state.error or (
                        f"RESPONSE_WRITE:{type(error).__name__}:{error}"
                    )
            finally:
                if state is not None:
                    state.done.set()


def load_probe(path: Path) -> ModuleType:
    """Load the frozen passive helper only after verifying its exact source."""
    raw = path.read_bytes()
    require(hashlib.sha256(raw).hexdigest() == PROBE_SHA, "FROZEN_PROBE_SHA")
    spec = importlib.util.spec_from_file_location("anfl_frozen_probe", path)
    require(spec is not None and spec.loader is not None, "FROZEN_PROBE_MODULE")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def observed_client_type(probe: ModuleType) -> type[Any]:
    """Reuse the frozen bounded lifecycle, retaining terminal notifications."""

    class ObservedClient(probe.NativeClient):
        def __init__(self, *args: Any, **kwargs: Any) -> None:
            self.observed: list[dict[str, Any]] = []
            super().__init__(*args, **kwargs)

        def pump(self, timeout: float) -> list[dict[str, Any]]:
            messages = super().pump(timeout)
            self.observed.extend(
                item for item in messages if "method" in item
            )
            require(len(self.observed) <= 512, "NOTIFICATION_CAP")
            return messages

    return ObservedClient


def check_source(path: Path, initial: bytes) -> dict[str, Any]:
    """Verify this case's immutable prefix and native ordinal chain."""
    require(path.stat().st_size <= MAX_BODY, "SYNTHETIC_FILE_CAP")
    raw = path.read_bytes()
    require(raw.startswith(initial) and raw.endswith(b"\n"), "PREFIX_OR_EOF")
    rows = [json.loads(line) for line in raw.splitlines()]
    require(len(rows) <= 1024, "SYNTHETIC_RECORD_CAP")
    require([row.get("ordinal") for row in rows] == list(range(len(rows))),
            "NATIVE_ORDINAL_CHAIN")
    return {"bytes": len(raw), "records": len(rows),
            "sha256": hashlib.sha256(raw).hexdigest()}


def one_case(
    probe: ModuleType, client_type: type[Any], binary: Path,
    root: Path, workspace: Path, catalog: Path, env: dict[str, str],
    codex: Path, selected: Path, name: str, initial: bytes,
) -> dict[str, Any]:
    """Own one receiver, native process and independently initialized baseline."""
    sqlite = root / f"sqlite-{name}"
    sqlite.mkdir(mode=0o700)
    require(not selected.exists(), "CASE_PATH_ALREADY_PRESENT")
    selected.write_bytes(initial)
    selected.chmod(0o600)
    model = json.loads(catalog.read_text())["models"][0]["slug"]
    state = Capture(name)
    receiver = Receiver(("127.0.0.1", 0), Handler)
    receiver.active = state
    thread = threading.Thread(target=receiver.serve_forever, daemon=True)
    started = False
    client: Any = None
    closure: dict[str, Any] | None = None
    primary: Exception | None = None
    cleanup_errors: list[str] = []
    receiver_closed = False
    completed: dict[str, Any] | None = None
    try:
        thread.start()
        started = True
        profile = root / f"native-{name}.sb"
        profile.write_text(
            probe.sandbox_profile(root, codex) +
            "(allow network-outbound (remote ip " +
            json.dumps(f"localhost:{receiver.server_port}") + "))\n"
        )
        canary_root = root / f"canary-{name}"
        (canary_root / "sqlite").mkdir(parents=True, mode=0o700)
        probe.verify_network_denial(profile, env, canary_root)
        canary = (
            "import urllib.request,sys\n"
            "with urllib.request.urlopen(sys.argv[1],timeout=5) as r:"
            "assert r.status==200\n"
        )
        subprocess.run(
            ["/usr/bin/sandbox-exec", "-f", str(profile), sys.executable,
             "-I", "-B", "-c", canary,
             f"http://127.0.0.1:{receiver.server_port}/canary"],
            env=env, capture_output=True, timeout=10, check=True,
        )
        emit("EXACT_LOOPBACK_PORT_CANARY_OK", case=name,
             other_ipv4_tcp_udp_ports_denied=True,
             ipv6_empirically_tested=False)
        args = probe.config_args(root, sqlite, catalog)
        url = f"http://127.0.0.1:{receiver.server_port}/v1"
        args += ["-c", "model_providers.anfl_public_offline.base_url=" +
                 json.dumps(url)]
        command = [
            "/usr/bin/sandbox-exec", "-f", str(profile), str(binary)
        ] + args + ["app-server", "--strict-config", "--listen", "stdio://"]
        client = client_type(command, env, workspace)
        probe.initialize(client)
        probe.resume(client, True, selected, workspace, model)
        reply = client.rpc("turn/start", {
            "threadId": probe.THREAD_ID,
            "input": [{"type": "text", "text": REQUEST, "textElements": []}],
        }, timeout=15)
        require("result" in reply, "SYNTHETIC_TURN_START_FAILED")
        turn_id = reply["result"]["turn"]["id"]
        deadline = time.monotonic() + 30
        while time.monotonic() < deadline:
            client.pump(0.1)
            if state.error is not None:
                raise RuntimeError("CAPTURE_ERROR:" + state.error)
            terminal = [
                item for item in client.observed
                if item.get("method") == "turn/completed"
                and item.get("params", {}).get("threadId") == probe.THREAD_ID
                and item.get("params", {}).get("turn", {}).get("id") == turn_id
            ]
            if state.done.is_set() and terminal:
                completed = terminal[-1]["params"]["turn"]
                break
            require(client.process.poll() is None, "NATIVE_EARLY_EXIT")
        require(state.done.is_set(), "HTTP_CAPTURE_DEADLINE")
        require(completed is not None, "TERMINAL_TURN_DEADLINE")
        require(completed.get("status") == "failed", "TURN_NOT_EXPECTED_FAILED")
    except Exception as error:
        primary = error
    finally:
        if client is not None:
            try:
                closure = client.close()
            except Exception as error:
                cleanup_errors.append(f"NATIVE_CLOSE:{error}")
        try:
            if started:
                receiver.shutdown()
        except Exception as error:
            cleanup_errors.append(f"RECEIVER_SHUTDOWN:{error}")
        try:
            receiver.server_close()
        except Exception as error:
            cleanup_errors.append(f"RECEIVER_CLOSE:{error}")
        if started:
            thread.join(timeout=5)
        receiver_closed = not thread.is_alive() and receiver.fileno() == -1
        emit("CASE_RECEIVER_CLOSED", case=name, observed=receiver_closed,
             captured_post_requests=state.calls,
             fixed_response_written=state.fixed_response_written,
             get_requests=receiver.get_calls, error=state.error,
             cleanup_errors=cleanup_errors)
    if primary is not None:
        stderr = getattr(primary, "stderr", None)
        stderr_tail = (
            stderr[-8192:].decode(errors="replace")
            if isinstance(stderr, bytes) else str(stderr or "")[-8192:]
        )
        emit("CASE_FAILURE", name=name, error=str(primary),
             exit_code=getattr(primary, "returncode", None),
             stderr_tail=stderr_tail,
             notification_methods=(
                 [x.get("method") for x in client.observed]
                 if client is not None else []
             ))
        raise primary
    require(not cleanup_errors and receiver_closed, "CASE_RECEIVER_NOT_CLOSED")
    require(closure is not None and closure["exit"] == 0 and
            not closure["forced"] and closure["group_gone"],
            "CASE_NATIVE_CLOSURE_NOT_QUALIFIED")
    require(state.done.is_set() and state.calls == 1 and
            state.error is None and state.fixed_response_written,
            "FINAL_RECEIVER_ACCOUNTING")
    require(receiver.get_calls == 1, "UNEXPECTED_GET_REQUEST_COUNT")
    require(state.payload is not None, "CAPTURE_PAYLOAD_MISSING")
    require(state.payload.get("model") == model, "WIRE_MODEL_MISMATCH")
    values = state.payload["input"]
    encoded = canonical(values)
    comparison, normalized_ids = normalized_input(values)
    summary = {
        "name": name, "input": values,
        "input_sha256": hashlib.sha256(encoded).hexdigest(),
        "comparison_sha256": hashlib.sha256(comparison).hexdigest(),
        "comparison_bytes": len(comparison),
        "normalized_message_ids": normalized_ids,
        "input_bytes": len(encoded), "input_items": len(values),
        "model": model, "captured_post_requests": state.calls,
        "fixed_response_writes": int(state.fixed_response_written),
        "canary_get_requests": receiver.get_calls,
        "native_turn_status": completed["status"] if completed else None,
        "prefix_present": PREFIX.encode() in encoded,
        "replacement_present": probe.REPLACEMENT_MARKER.encode() in encoded,
        "tail_present": TAIL.encode() in encoded,
        "request_present": REQUEST.encode() in encoded,
        "closure": closure, "receiver_closed": receiver_closed,
        "source_after": check_source(selected, initial),
    }
    selected.rename(root / f"{name}-native-result.jsonl")
    emit("NATIVE_WIRE_CASE_COMPLETE", **{
        key: value for key, value in summary.items() if key != "input"
    })
    return summary


def run(probe_path: Path) -> None:
    """Run only on the authorized disposable public macOS ARM64 runner."""
    require(os.environ.get("GITHUB_ACTIONS") == "true", "REMOTE_CI_ONLY")
    require(os.environ.get("GITHUB_REPOSITORY") ==
            "MattSStack/anfl-public-validation", "PUBLIC_REPOSITORY_ONLY")
    require(platform.system() == "Darwin" and platform.machine() == "arm64",
            "MACOS_ARM64_ONLY")
    require(sys.version_info >= (3, 11), "PYTHON_VERSION")
    require("CODEX_HOME" not in os.environ and
            "CODEX_SQLITE_HOME" not in os.environ, "INHERITED_NATIVE_ROOT")
    codex = Path(os.environ["HOME"]) / ".codex"
    require(not codex.exists() and not Path("/etc/codex").exists(),
            "NATIVE_STATE_NOT_FRESH")
    probe = load_probe(probe_path)
    root = Path(tempfile.mkdtemp(
        prefix="anfl-public-wire-", dir=os.environ["RUNNER_TEMP"]
    )).resolve()
    root.chmod(0o700)
    codex.mkdir(mode=0o700)
    workspace = root / "workspace"
    workspace.mkdir(mode=0o700)
    child_tmp = root / "child-tmp"
    child_tmp.mkdir(mode=0o700)
    archive, catalog, binary = (
        root / "release.tar.gz", root / "models.json", root / "codex"
    )
    probe.fetch_pin(probe.ASSET_URL, archive, probe.ASSET_BYTES, probe.ASSET_SHA)
    probe.fetch_pin(probe.CATALOG_URL, catalog,
                    probe.CATALOG_BYTES, probe.CATALOG_SHA)
    binary_pin = probe.extract_binary(archive, binary)
    env = probe.safe_environment()
    env["TMPDIR"] = str(child_tmp)
    parent = codex / "sessions" / "2026" / "10" / "22"
    parent.mkdir(parents=True, mode=0o700)
    selected = parent / (
        f"rollout-2026-10-22T00-00-00-{probe.THREAD_ID}.jsonl"
    )
    client_type = observed_client_type(probe)
    baselines = cases(probe.fixture_bytes(str(workspace)))
    summaries: dict[str, dict[str, Any]] = {}
    for name, initial in baselines.items():
        summaries[name] = one_case(
            probe, client_type, binary, root, workspace, catalog,
            env, codex, selected, name, initial,
        )
    full, bounded, negative = (
        summaries["full"], summaries["bounded"], summaries["negative"]
    )
    full_bytes, _ = normalized_input(full["input"])
    bounded_bytes, _ = normalized_input(bounded["input"])
    negative_bytes, _ = normalized_input(negative["input"])
    require(full_bytes == bounded_bytes, "FULL_BOUNDED_INPUT_DIFFER")
    require(full_bytes != negative_bytes, "NEGATIVE_INPUT_UNCHANGED")
    require(not full["prefix_present"] and not bounded["prefix_present"],
            "CHECKPOINT_FAILED_TO_REPLACE_PREFIX")
    require(negative["prefix_present"], "NEGATIVE_PREFIX_NOT_OBSERVED")
    require(full["replacement_present"] and bounded["replacement_present"]
            and not negative["replacement_present"], "CHECKPOINT_MARKERS")
    require(all(row["tail_present"] and row["request_present"]
                for row in summaries.values()), "TAIL_OR_REQUEST_MISSING")
    emit(
        "NATIVE_SYNTHETIC_MODEL_INPUT_QUALIFIED",
        release="0.163.0", upstream_commit=probe.REVISION,
        frozen_probe_sha256=PROBE_SHA, release_binary=binary_pin,
        full_bounded_comparison_equal=True, negative_comparison_differs=True,
        raw_full_bounded_input_equal=(
            canonical(full["input"]) == canonical(bounded["input"])
        ),
        comparison=(
            "Canonical ordered input JSON; only declared unreferenced fresh "
            "instruction/current-request message IDs normalized. "
            "All fixed historical IDs and remaining values preserved."
        ),
        cases={name: {k: v for k, v in row.items() if k != "input"}
               for name, row in summaries.items()},
        requested_synthetic_turns=sum(
            method == "turn/start" for row in summaries.values()
            for method in row["closure"]["requested_methods"]
        ),
        fixed_response_writes=sum(
            row["fixed_response_writes"] for row in summaries.values()
        ),
        provider_behavior="Loopback receiver returns fixed400; executes no model",
        scope="Plaintext custom-provider wire input only",
        openai_provider_equivalence_proven=False,
        hidden_reconstruction_state_proven=False,
        opaque_ciphertext_semantics_proven=False,
        actual_checkpoint_proven=False, retirement_authorized=False,
    )


def self_test() -> None:
    """Check duplicate refusal and fixture boundaries without native execution."""
    try:
        json.loads('{"input":[],"input":[]}', object_pairs_hook=unique_object)
    except RuntimeError as error:
        require(str(error) == "DUPLICATE_JSON_KEY", "DUPLICATE_GUARD")
    else:
        raise RuntimeError("DUPLICATE_ACCEPTED")
    require(canonical([{"z": 1, "a": 2}]) == b'[{"a":2,"z":1}]',
            "OBJECT_KEYS_CANONICAL")
    require(canonical([1, 2]) != canonical([2, 1]), "ORDER_MUST_NOT_NORMALIZE")
    require(canonical([True]) != canonical([1]), "BOOL_INT_DISTINCT")
    require(canonical([1]) != canonical([1.0]), "INT_FLOAT_DISTINCT")
    first = [{"type": "message", "role": "user", "id": "msg_fresh_a",
              "content": [{"type": "input_text", "text": REQUEST}]}]
    second = json.loads(json.dumps(first))
    second[0]["id"] = "msg_fresh_b"
    require(normalized_input(first)[0] == normalized_input(second)[0],
            "DECLARED_FRESH_ID_COMPARISON")
    second[0]["content"][0]["text"] += " changed"
    require(normalized_input(first)[0] != normalized_input(second)[0],
            "CONTENT_CHANGE_MUST_SURVIVE")
    referenced = first + [{"type": "item_reference", "id": "msg_fresh_a"}]
    try:
        normalized_input(referenced)
    except RuntimeError as error:
        require(str(error) == "MESSAGE_ID_IS_REFERENCED", "REFERENCE_GUARD")
    else:
        raise RuntimeError("REFERENCED_ID_NORMALIZED")
    emit("PURE_INPUT_CONTRACT_CHECKS_OK", native_processes=0)


def main() -> int:
    """Select pure checks or explicitly guarded remote-only execution."""
    parser = argparse.ArgumentParser()
    parser.add_argument("--probe", type=Path)
    parser.add_argument("--self-test", action="store_true")
    options = parser.parse_args()
    try:
        if options.self_test:
            self_test()
        else:
            require(options.probe is not None, "FROZEN_PROBE_REQUIRED")
            run(options.probe)
    except Exception as error:
        emit("ERROR", exception=type(error).__name__, detail=str(error))
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
