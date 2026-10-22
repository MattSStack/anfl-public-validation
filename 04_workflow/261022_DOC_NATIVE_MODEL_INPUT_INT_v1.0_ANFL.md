<!-- ANFL | The Reactor™ | Layer 04 | Version 1.0 | 2026-10-22 -->
# Released native synthetic model-input qualification

BLUF: Observe the released Codex 0.163.0 model-request input for three public
synthetic histories: full, bounded checkpoint suffix, and missing checkpoint.
This distinct test requests three synthetic turns against a loopback-only
receiver that executes no model and returns fixed HTTP 400. Three native captures
and clean closures are observed; the explicit environment-ID comparison successor
is awaiting qualification.

## Executive

The existing native consumer test qualified projection readback, source-absence
refusal and cold resume. Persisted history projections do not expose reconstructed
model context. This test observes the actual custom-provider HTTP request instead
of adding a reconstruction surrogate or rebuilding codex-core.

The native source already has full-versus-bounded reconstruction unit tests.
This separate test adds released-binary wire-input evidence for controlled
plaintext fixtures. It cannot establish actual checkpoint acceptance, hidden
state equality, OpenAI-provider behavior or retirement authority.

## Implementation Status

```yaml
component: "Released native synthetic model-input qualification"
status: "in_progress"
version: "1.0"
position_classification: "B"
layer: "06"
progress_percentage: 0
dependencies:
  - "Finite independent proof and lifecycle reviews"
  - "Actual public remote execution"
```

## Architecture

```text
Pinned official release + catalog + frozen native helper
                            |
       Shared fresh disposable runner HOME / controlled cwd
                            |
          +-----------------+------------------+
          |                 |                  |
   Full prefix+CP+tail  Header+sameCP+tail   CP removed
          |                 |                  |
   Fresh SQLite + own selected synthetic baseline per case
          |                 |                  |
   Own exact-port receiver + network canaries + native resume
          |                 |                  |
   Synthetic turn -> bounded POST input -> fixed400 -> failed turn
          |                 |                  |
   Close native and receiver -> stable final counters and hashes
          +-----------------+------------------+
                            |
        Canonical-byte comparison + explicit fresh-ID map
                            |
             Full == Bounded; Negative != Full
```

## Implementation

### Ordered Change Set and pins

1. `06_api_integration/261022_PY_NATIVE_MODEL_INPUT_INT_v1.0_ANFL.py`:
   28867 bytes; SHA256 `5ce97a732df8c16e5de97e6e18adc75e2e0edcb71febe046d827945e185b9d7c`.
2. `01_infrastructure/261022_YAML_NATIVE_MODEL_INPUT_CI_INT_v1.0_ANFL.yml`:
   2487 bytes; SHA256 `edd1286813a5d51d7b7d26da28137aeb3fc6c903da59fd79270dcd8bc133d20f`.
   Mirror the exact workflow bytes under `.github/workflows/`.
3. This complete review contains both implementation files.
4. The existing recovery-support ledger records admission intent and run outcomes.
   Its local snapshot below is excluded from public publication.

The existing native consumer helper is imported passively and remains frozen:
`261022_PY_NATIVE_CONSUMER_PROBE_INT_v1.0_ANFL.py`, SHA256
`f0e3d7a5bde0e09e4953c23474bc5039c03a2aeb0d5f0db3c1fe9d2dae64ad1f`.

Official source: `83086188ae0614bd882818ff1aa23f3194962012`.
Release archive: asset 645204306, 99,551,880 bytes, SHA256
`61c4d9a92521bdb42b0c1d9f87c673f6da628a06de5945cbdd9a0010632f6d10`.
Catalog: 472,072 bytes, SHA256
`943ca7fe1d19ed019054158303f0dbbd80b3bef43b3aa425a71aa7cb3fb52c2b`.
The frozen helper independently checks these exact remote downloads.

### Observation and comparison contract

Each case uses the same controlled cwd, thread identity, catalog/model, feature
settings and default HOME. Each gets a fresh SQLite directory, selected baseline,
native process, receiver and port. Close and move the prior synthetic result
outside the managed sessions tree before installing the next baseline.

The receiver accepts one bounded uncompressed JSON POST to `/v1/responses`.
No authorization header is accepted. It returns a generic fixed 400, avoiding
auth recovery, rate-limit/server retries and context-length error recovery.
Request and stream retry counts are zero. Exactly one observed POST, one
completed local fixed-response write and one canary GET are required per case.
Successful local response writes do not prove peer acknowledgment.

Compare canonical serialized input bytes, retaining array order and scalar
distinctions. Object-key order is canonicalized. Persisted fixture message IDs
are fixed and preserved. Separately generated developer/system/current-request
and current-environment message IDs may differ between turns; normalize only
their declared top-level
`id` fields after proving their string token is unreferenced elsewhere.
Report each changed index, role and raw ID. Preserve all content, types, roles,
call IDs, references, opaque values and other fields. Keep the canonical
pre-ID-normalization hash and separately report raw-input equality.


The current-environment exception is fixture-specific. Require exact message keys,
user role, one input_text block with exact type/text keys, the complete native
environment_context wrapper, no fixture markers, and penultimate position directly
before the exact current-request block. Normalize only its unreferenced ID.
Every byte of environment content remains in the comparison. The wrapper is not
a general provenance or authentication proof.

The positive cases must match after this declared normalization. The negative
must differ and restore the older-prefix sentinel. The checkpoint replacement,
tail and new-request sentinels must appear in the expected cases.
No semantic claim follows merely from equal hashes without these gates.

### Source evidence and applicability limits

[Native request construction](https://github.com/openai/codex/blob/83086188ae0614bd882818ff1aa23f3194962012/codex-rs/core/src/client.rs)
formats prompt input;
[HTTP endpoint](https://github.com/openai/codex/blob/83086188ae0614bd882818ff1aa23f3194962012/codex-rs/codex-api/src/endpoint/responses.rs)
serializes the Responses request. The
[upstream capture fixtures](https://github.com/openai/codex/blob/83086188ae0614bd882818ff1aa23f3194962012/codex-rs/core/tests/common/responses.rs)
inspect this same input boundary.

For the authless custom provider, request compression is absent on this path.
Custom-provider formatting strips internal metadata and some encrypted fields;
remaining ciphertext is opaque. Therefore this qualifies controlled plaintext
custom-provider input only. It is not full native RolloutReconstruction equality,
OpenAI-provider equivalence or interpretation of encrypted checkpoint state.
The
[existing native reconstruction tests](https://github.com/openai/codex/blob/83086188ae0614bd882818ff1aa23f3194962012/codex-rs/core/src/session/rollout_reconstruction_tests.rs)
already test current/legacy Paginated full-versus-bounded results and missing
companion behavior. They are not rerun by this workflow.

The pinned [environment renderer](https://github.com/openai/codex/blob/83086188ae0614bd882818ff1aa23f3194962012/codex-rs/core/src/context/world_state/environment.rs)
produces the environment context as a user fragment. The
[fragment conversion](https://github.com/openai/codex/blob/83086188ae0614bd882818ff1aa23f3194962012/codex-rs/context-fragments/src/fragment.rs)
creates one input-text message with an initially absent ID; missing IDs are
assigned in the native session path. This supports the explicit fixture-only
classification, while preserving content and order in the comparison.

### Threat notes

The workflow is manual, public-repository-only, standard macos-15 ARM64, and
contents:read. It supplies no operator input, credential or key. Native children
receive a small environment allowlist, unchanged HOME, controlled TMPDIR,
explicit SQLite root, static catalog, file credential storage, disabled startup
hooks/plugins/snapshots/skill instructions and a credential-free custom provider.

Each native child is OS-denied network access except outbound traffic to its
one exact localhost receiver port. The receiver binds IPv4 only. Working positive controls and denied
IPv4 TCP/UDP bind/outbound attempts precede it. A permitted-port GET verifies
the exception. IPv6 loopback at the same port is within the declared exception but is not
empirically tested. Other destinations remain denied.
The Python parent has network access for pinned public downloads and binds only
loopback for capture. Its receiver executes no model. The complete job is not
an air-gapped environment.

A fixed error alone does not neutralize startup hooks or tools. Fresh default
state, controlled empty cwd, static configuration and the OS envelope remain
necessary. The read-permissive sandbox does not attest every read. The trusted
fresh public runner is the input boundary; no local Mac Codex process runs.

HTTP body is capped at 1 MiB before allocation. Native stdout/stderr bounds and
transactional process ownership come from the frozen helper. Each case requires
native exit0/no force/group absence, receiver socket close/thread join, then
stable counters and errors. Socket-operation timeouts and client deadlines are
not complete hard wall-time limits. The ephemeral job has a 12-minute timeout.
No actual recovery input or retirement follows from this test.

### Validation and Test Plan

Pure checks passed: duplicate JSON key refusal; ordered arrays; canonical object
keys; Boolean/integer and integer/float distinctions; only declared fresh IDs
normalize; changed content remains different; referenced IDs refuse normalization.
No native process was started for these local checks.

Independent reviews corrected raw Python equality, fresh native IDs, pre-close
counter snapshots, response-write accounting, receiver ownership and catalog
lookup ordering. Pending runtime gates: three exact-port canaries, three native
captures/expected failed turns, clean native and receiver closures, positive
equality, negative inequality and marker checks. Do not silently relax a failed
comparison or broaden the network profile.

### Retained first runtime failure and compiler diagnosis

[Run 39776595182](https://github.com/MattSStack/anfl-public-validation/actions/runs/39776595182),
attempt 1, job119817002402, commit
`b2c1bed4b5fbbe524bdbb4694e970f5c9928fde9`: failed before any Codex launch.
Pinned public downloads passed. The per-case receiver closed with zero requests;
Seatbelt's canary child exited65 while compiling the numeric-host filter.

A local compiler-only diagnostic used `/usr/bin/true`, without Codex, sockets or
file writes. Numeric `127.0.0.1:port` reported:
`host must be * or localhost in network address`.
The official-source spelling `localhost:port` compiled and the child exited0.
The successor corrects only that endpoint-filter spelling and retains bounded
stderr diagnostics. It does not widen access to any external network.
Native initialization, model input and comparison remain unqualified here.

### Retained first native-capture result

[Run 39777274107](https://github.com/MattSStack/anfl-public-validation/actions/runs/39777274107),
attempt1/job119819310414, commit
`d851d3aaf04f2193eca19202bb72f4e1ce601411`: canaries, native initialize/resume,
one POST, one fixed400 write, expected failed turn, native exit0/group absence,
and receiver closure passed for Full. Normalization then refused an undeclared
message ID. The actual refusing item was not logged; no comparison qualified.

The successor retains bounded per-item identity, role, content hash/prefix and
marker diagnostics before normalization. It collects the same three admitted
cases before pure comparison, without changing fixtures, normalization rules,
acceptance gates or network scope. A refusal remains a failed qualification.
Source review does not support guessing historical-ID clearing or user-message
merging on the inspected replay/formatting paths.

### Retained diagnostic run and explicit rule

[Run 39778764415](https://github.com/MattSStack/anfl-public-validation/actions/runs/39778764415),
attempt1/job119824357765, commit
`33800e77251aff8ec1a1677064b017fabcf91c16`: all three native and receiver
closures passed. Each case captured one POST and completed one fixed400 write.
Full and bounded inputs contained replacement/tail/request, without older prefix;
the missing-checkpoint case restored older prefix and omitted replacement.

Comparison refused a newly generated current environment-context user-message ID.
Its complete content had identical 544-byte canonical content and SHA256
`138afa231eba1c02f17cfa76ddb58e8f902dc2ab3673af8dc712d262cc6f5795`
across the three cases. Historical fixture IDs were preserved. This is a retained
failure, not a qualified equality result. The successor explicitly declares the
narrow environment-ID class described above and adds pure tests for changed
content, missing/current-request position and unknown-message refusal.

### Full model-input helper

```python
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
REPLACEMENT = "ANFL_PUBLIC_SYNTHETIC_REPLACEMENT_261022"
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

    require(
        rows[1]["payload"]["replacement_history"][0]["content"] == [
            {"type": "input_text", "text": REPLACEMENT}
        ], "QUALIFIED_REPLACEMENT_MARKER",
    )
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
    """Normalize declared fresh instruction/request/environment message IDs.

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
        current_request = (
            role == "user" and REQUEST.encode() in content
            and PREFIX.encode() not in content and TAIL.encode() not in content
        )
        blocks = item.get("content")
        environment = False
        if (
            role == "user"
            and set(item) == {"type", "role", "id", "content"}
            and isinstance(blocks, list)
            and len(blocks) == 1
            and isinstance(blocks[0], dict)
            and set(blocks[0]) == {"type", "text"}
            and blocks[0].get("type") == "input_text"
            and isinstance(blocks[0].get("text"), str)
            and index == len(copied) - 2
        ):
            text = blocks[0]["text"]
            following = copied[index + 1]
            environment = (
                text.startswith("<environment_context>\n")
                and text.endswith("\n</environment_context>")
                and all(marker not in text
                        for marker in (PREFIX, TAIL, REPLACEMENT, REQUEST))
                and following.get("type") == "message"
                and following.get("role") == "user"
                and following.get("content") == [
                    {"type": "input_text", "text": REQUEST}
                ]
            )
        allowed = role in ("developer", "system") or current_request or environment
        require(allowed, "UNDECLARED_MESSAGE_ID_NORMALIZATION")
        require(raw.count(canonical(identity)) == 1, "MESSAGE_ID_IS_REFERENCED")
        item["id"] = f"ANFL_PUBLIC_VOLATILE_MESSAGE_ID_AT_{index}"
        changes.append({"index": index, "field": "id",
                        "role": role, "raw_id": identity,
                        "class": "current_environment" if environment else
                        "current_request" if current_request else "instruction"})
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
    require(len(values) <= 128, "CAPTURE_ITEM_DIAGNOSTIC_CAP")
    diagnostics = []
    for index, item in enumerate(values):
        content = canonical(item.get("content", []))
        diagnostics.append({
            "index": index, "type": item.get("type"), "role": item.get("role"),
            "id": item.get("id"), "keys": sorted(item.keys()),
            "content_bytes": len(content),
            "content_sha256": hashlib.sha256(content).hexdigest(),
            "content_prefix": content[:256].decode(errors="replace"),
            "prefix_marker": PREFIX.encode() in content,
            "replacement_marker": probe.REPLACEMENT_MARKER.encode() in content,
            "tail_marker": TAIL.encode() in content,
            "request_marker": REQUEST.encode() in content,
        })
    summary = {
        "name": name, "input": values,
        "input_sha256": hashlib.sha256(encoded).hexdigest(),
        "item_diagnostics": diagnostics,
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
    for name, row in summaries.items():
        comparison, normalization = normalized_input(row["input"])
        row["comparison_sha256"] = hashlib.sha256(comparison).hexdigest()
        row["comparison_bytes"] = len(comparison)
        row["normalized_message_ids"] = normalization
        emit("INPUT_COMPARISON_PREPARED", case=name,
             comparison_sha256=row["comparison_sha256"],
             comparison_bytes=row["comparison_bytes"],
             normalized_message_ids=normalization)
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
            "instruction/current-request/current-environment IDs normalized. "
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
    environment = {
        "type": "message", "role": "user", "id": "msg_environment_a",
        "content": [{"type": "input_text",
                     "text": "<environment_context>\n"
                     "  <cwd>/public/test</cwd>\n</environment_context>"}],
    }
    pair = [environment, first[0]]
    alternate = json.loads(json.dumps(pair))
    alternate[0]["id"] = "msg_environment_b"
    require(normalized_input(pair)[0] == normalized_input(alternate)[0],
            "DECLARED_ENVIRONMENT_ID_COMPARISON")
    alternate[0]["content"][0]["text"] = (
        "<environment_context>\n  <cwd>/changed</cwd>\n</environment_context>"
    )
    require(normalized_input(pair)[0] != normalized_input(alternate)[0],
            "ENVIRONMENT_CONTENT_CHANGE_MUST_SURVIVE")
    for malformed in (
        [environment],
        [{**environment, "content": [{"type": "input_text",
          "text": "<environment_context>\\n" + REPLACEMENT +
          "\\n</environment_context>"}]}, first[0]],
        [environment, {"type": "message", "role": "user", "content": []}],
        [{"type": "message", "role": "user", "id": "msg_unknown",
          "content": [{"type": "input_text", "text": "unclassified"}]}, first[0]],
    ):
        try:
            normalized_input(malformed)
        except RuntimeError as error:
            require(str(error) == "UNDECLARED_MESSAGE_ID_NORMALIZATION",
                    "ENVIRONMENT_POSITION_GUARD")
        else:
            raise RuntimeError("UNDECLARED_ENVIRONMENT_ID_NORMALIZED")
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
```

### Full manual workflow

```yaml
# ANFL | The Reactor™ | Layer 01 | Version 1.0 | 2026-10-22
name: ANFL public native model input
run-name: Native model input ${{github.run_number}}
on:
  workflow_dispatch:
permissions:
  contents: read
concurrency:
  group: anfl-public-native-model-input
  cancel-in-progress: false
jobs:
  qualify:
    if: >-
      github.repository == 'MattSStack/anfl-public-validation' &&
      github.event.repository.private == false
    runs-on: macos-15
    timeout-minutes: 12
    steps:
      - name: Fetch pinned public helpers and compare synthetic native inputs
        shell: python
        env:
          ANFL_INPUT_REVISION: ${{github.sha}}
          ANFL_INPUT_SHA256: 5ce97a732df8c16e5de97e6e18adc75e2e0edcb71febe046d827945e185b9d7c
        run: |
          import hashlib
          import os
          import pathlib
          import subprocess
          import sys
          import tempfile
          import urllib.request

          root = pathlib.Path(tempfile.mkdtemp(
              prefix="anfl-input-helpers-", dir=os.environ["RUNNER_TEMP"]
          ))
          specs = [
              (
                  "261022_PY_NATIVE_CONSUMER_PROBE_INT_v1.0_ANFL.py",
                  "f0e3d7a5bde0e09e4953c23474bc5039c03a2aeb0d5f0db3c1fe9d2dae64ad1f",
              ),
              (
                  "261022_PY_NATIVE_MODEL_INPUT_INT_v1.0_ANFL.py",
                  os.environ["ANFL_INPUT_SHA256"],
              ),
          ]
          for name, expected in specs:
              url = (
                  "https://raw.githubusercontent.com/"
                  "MattSStack/anfl-public-validation/"
                  + os.environ["ANFL_INPUT_REVISION"]
                  + "/06_api_integration/" + name
              )
              with urllib.request.urlopen(url, timeout=30) as response:
                  data = response.read(65537)
              if len(data) > 65536:
                  raise RuntimeError("PUBLIC_HELPER_SIZE_LIMIT")
              if hashlib.sha256(data).hexdigest() != expected:
                  raise RuntimeError("PUBLIC_HELPER_SHA256")
              (root / name).write_bytes(data)
          helper = root / specs[1][0]
          probe = root / specs[0][0]
          subprocess.run(
              [sys.executable, "-B", str(helper), "--self-test"],
              check=True, timeout=20,
          )
          subprocess.run(
              [sys.executable, "-B", str(helper), "--probe", str(probe)],
              check=True, timeout=680,
          )
```

## Ops / Runbook

Record public commit, workflow ID, one dispatch intent, run ID/attempt and job ID.
Monitor only that owned run. Retain any failure and its primary/cleanup evidence;
reconcile unknown outcomes before any new attempt. This is a new synthetic turn
test, not a replay of the prior no-turn consumer qualification.

Standard public GitHub-hosted runner use is documented free. No artifact/cache,
release upload, larger runner, persistent server or billable external service is
created.
[GitHub Actions free use](https://docs.github.com/en/billing/concepts/product-billing/github-actions#free-use-of-github-actions).

Rollback: leave the workflow undispatched or cancel only its exact owned run;
disable/revert additive public files if needed. The frozen consumer/scanner and
all actual recovery data, owners, windows and control records remain preserved.
Disposable runner test state has no recovery-custody role.
