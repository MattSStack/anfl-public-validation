<!-- ANFL | The Reactor™ | Layer 04 | Version 1.0 | 2026-10-22 -->
# Public synthetic native consumer qualification

BLUF: This probe tests stable-ID consumption of one tiny managed Paginated
rollout by the official released Codex 0.163.0 ARM64 binary. Execution remains
pending. It does not qualify a real recovery checkpoint or retirement.

## Executive

Use only the existing public validation repository and a disposable standard
public macOS ARM64 runner. The fixture contains five public synthetic records.
The native child receives no operator credential or model-turn request.
Network denial is enforced by Seatbelt and tested with positive and negative
TCP/UDP canaries; writable SQLite WAL state is tested separately.

This adds released-binary front-door evidence to public integration-test source
review. It is a managed-path stable-ID experiment, not an external-path import.

## Implementation Status

```yaml
component: "Public synthetic native consumer qualification"
status: "in_progress"
version: "1.0"
position_classification: "B"
layer: "06"
progress_percentage: 0
dependencies:
  - "Independent lifecycle and proof review"
  - "Actual remote canaries and released-binary execution"
```

## Architecture

```text
Pinned official release + public catalog
                    |
Fresh public runner / unchanged default home / explicit SQLite root
                    |
OS network denial + TCP/UDP controls + SQLite WAL canary
                    |
Native stdio: initialize -> missing-ID refusal
                    |
Five-record managed fixture -> resume -> projected marker
                    |
Close -> hold selected source outside sessions -> fresh refusal
                    |
Restore synthetic filename -> cold resume + marker -> close
                    |
Qualified narrow result or retained failure; no automatic retry
```

## Implementation

### Ordered Change Set

1. `06_api_integration/261022_PY_NATIVE_CONSUMER_PROBE_INT_v1.0_ANFL.py`
   Full standalone remote-only probe: 31016 bytes; SHA256
   `05a7b1775e91bc889053564a06336119ec43e55b86c1bc696655b9dd91e4981b`.
2. `01_infrastructure/261022_YAML_NATIVE_CONSUMER_CI_INT_v1.0_ANFL.yml`
   Manual-only public-runner workflow: 2116 bytes; SHA256
   `adfbe46e6b881b34e687dc0a352fc607b3f8187ebe72f79a744670b3a91d812f`.
   The same exact bytes are mirrored under `.github/workflows/`.
3. This full-content review records the contract, complete source and workflow.

### Exact public provenance

- Official source commit: `83086188ae0614bd882818ff1aa23f3194962012`.
- Annotated release tag `rust-v0.163.0` resolves to that source commit.
  The tag reports unsigned; this is not a reproducible-build attestation.
- Release ID `414286110`, ARM64 archive asset `645204306`,
  `codex-aarch64-apple-darwin.tar.gz`, 99,551,880 bytes, SHA256
  `61c4d9a92521bdb42b0c1d9f87c673f6da628a06de5945cbdd9a0010632f6d10`.
  Remote download must independently match both size and digest before use.
- Catalog `codex-rs/models-manager/models.json`: 472,072 bytes; SHA256
  `943ca7fe1d19ed019054158303f0dbbd80b3bef43b3aa425a71aa7cb3fb52c2b`.
  The exact public catalog has 11 models. Startup selects its first model;
  no model inference is requested.

[Official release](https://github.com/openai/codex/releases/tag/rust-v0.163.0),
[fixture builder](https://github.com/openai/codex/blob/83086188ae0614bd882818ff1aa23f3194962012/codex-rs/app-server/tests/common/rollout.rs),
[stable-ID paginated test](https://github.com/openai/codex/blob/83086188ae0614bd882818ff1aa23f3194962012/codex-rs/app-server/tests/suite/v2/thread_resume.rs#L169).

### Observable acceptance and limits

Require exact selected ID, canonical managed path, cwd, root source, idle state,
history mode, effective provider and model. Independently list turns and items,
checking the one exact typed projected user marker and absent pagination.

The missing-source check must match native error code -32600 and the exact
synthetic-ID not-found message. Unrelated authentication or configuration errors
must fail qualification. Run a new process after holding the fixture outside the
managed sessions tree; keep the same synthetic SQLite state to test stale-row
refusal. Then restore only that synthetic filename and cold-resume it again.

Native resume is stateful. Require the original five-record prefix to remain
exact, final LF, bounded file extent, native appended records and a contiguous
ordinal chain. The cold file check describes cumulative append history, not a
claim of a separately measured new cold append.

The public projection ignores Compacted and raw ResponseItem records. The
readable projection marker is separate from the checkpoint replacement marker.
A successful resume traverses native reconstruction, but reconstruction has no
typed error result and some persistence/projection failures are logged or ignored.
Therefore resume plus marker does not establish replacement-history equivalence.
[Projection](https://github.com/openai/codex/blob/83086188ae0614bd882818ff1aa23f3194962012/codex-rs/app-server-protocol/src/protocol/thread_history_projection.rs#L78),
[resume persistence](https://github.com/openai/codex/blob/83086188ae0614bd882818ff1aa23f3194962012/codex-rs/core/src/session/mod.rs#L1671).

No large-history resource guarantee, complete-tail coverage, actual replacement
acceptance, independent custody, complete lineage or retirement follows.
The existing portable-restoration and full physical-acquisition obligations
remain open.

### Threat notes and validation

A fresh runner's unchanged default Codex home must be absent initially.
Inherited CODEX_HOME or CODEX_SQLITE_HOME fails the probe. The child receives a
small environment allowlist with unchanged HOME and a task-specific TMPDIR,
strict configuration loading, explicit sqlite_home, file credential storage and a custom provider without
credential fields. The workflow contains no secrets reference and grants only
contents:read. It uses no artifact, cache, release, Packages or LFS upload.

No-turn startup can still discover models or prewarm authentication. The static
catalog and custom provider reduce activity; the actual network boundary is
`(deny network*)`. Runtime canaries test IPv4 TCP/UDP bind and outbound attempts.
IPv6 is covered by that declared policy but is not empirically tested.
No denied helper IPC or filesystem access is silently widened.

The sandbox permits reads and is not an independent measure of all reads.
Fresh public-runner state and a public-input-only workflow provide the data
boundary. Environment guards are accidental-host checks, not independent
attestation of runner identity. No protected operator state is supplied.

Native stdout/stderr, line, record, fixture and download limits are finite.
A read can cross a capture threshold by one 65,536-byte chunk before refusal.
Native internal memory is not hard capped. Download and RPC deadlines are not
complete hard wall-time limits; read timeout and blocking writes qualify them.
The job has a 12-minute timeout on an ephemeral runner. Forced shutdown or
unobserved process-group closure fails qualification.

### Test Plan and completed local checks

- Thirteen memory-only fixture/profile assertions passed.
- Four controlled Python-child lifecycle cases passed: matching reply with
  notification and clean EOF; oversized output refusal with owned child reaped;
  RPC deadline followed by clean EOF; injected post-spawn selector failure with
  child and all pipes positively closed.
- These controlled children were not native Codex. No local native binary,
  local native app server, operator config or recovery source was used.
- Embedded workflow Python parsed successfully.
- Pending: actual remote release hash, envelope canaries, native source-absence
  refusal, positive typed projection, cold reopen and three clean closures.

### Full probe source

```python
#!/usr/bin/env python3
"""Qualify a public synthetic native consumer on a disposable macOS CI runner.

ANFL | The Reactor™ | Layer 06 | Version 1.0 | 2026-10-22
Implementation status: planned; execution receipts are maintained separately.
No operator input, model turn, private credentials, or local Mac runtime.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import platform
import selectors
import signal
import socket
import subprocess
import sys
import tarfile
import tempfile
import time
import urllib.request
import urllib.parse
from pathlib import Path
from typing import Any

REVISION = "83086188ae0614bd882818ff1aa23f3194962012"
ASSET_URL = (
    "https://github.com/openai/codex/releases/download/rust-v0.163.0/"
    "codex-aarch64-apple-darwin.tar.gz"
)
ASSET_BYTES = 99_551_880
ASSET_SHA = "61c4d9a92521bdb42b0c1d9f87c673f6da628a06de5945cbdd9a0010632f6d10"
CATALOG_URL = (
    f"https://raw.githubusercontent.com/openai/codex/{REVISION}/"
    "codex-rs/models-manager/models.json"
)
CATALOG_BYTES = 472_072
CATALOG_SHA = "943ca7fe1d19ed019054158303f0dbbd80b3bef43b3aa425a71aa7cb3fb52c2b"
THREAD_ID = "019a0cee-5a00-7a01-8b01-000000000001"
TURN_ID = "public-synthetic-turn-1"
PROJECTION_MARKER = "ANFL_PUBLIC_SYNTHETIC_PROJECTION_261022"
REPLACEMENT_MARKER = "ANFL_PUBLIC_SYNTHETIC_REPLACEMENT_261022"
MAX_CAPTURE = 2 * 1024 * 1024
MAX_LINE = 256 * 1024


def require(condition: bool, code: str) -> None:
    """Raise an explicit qualification failure when a condition is false."""
    if not condition:
        raise RuntimeError(code)


def emit(event: str, **fields: Any) -> None:
    """Write bounded structured public-only diagnostic evidence."""
    level = "ERROR" if event == "ERROR" else "INFO"
    print(json.dumps({"level": level, "event": event, **fields},
                     sort_keys=True), flush=True)


def fixture_bytes(cwd: str) -> bytes:
    """Return the five-record source-derived paginated synthetic fixture."""
    header = {
        "id": THREAD_ID,
        "session_id": THREAD_ID,
        "timestamp": "2026-10-22T00:00:00Z",
        "cwd": cwd,
        "originator": "anfl_public_synthetic",
        "cli_version": "0.163.0",
        "source": "cli",
        "history_mode": "paginated",
        "history_base": None,
    }
    payloads = [
        ("session_meta", header),
        (
            "compacted",
            {
                "message": "ANFL public synthetic checkpoint",
                "replacement_history": [
                    {
                        "type": "message",
                        "role": "user",
                        "content": [
                            {"type": "input_text", "text": REPLACEMENT_MARKER}
                        ],
                    }
                ],
                "window_number": 1,
            },
        ),
        (
            "event_msg",
            {
                "type": "task_started",
                "turn_id": TURN_ID,
                "model_context_window": None,
                "collaboration_mode_kind": "default",
            },
        ),
        (
            "event_msg",
            {
                "type": "item_completed",
                "thread_id": THREAD_ID,
                "turn_id": TURN_ID,
                "item": {
                    "type": "UserMessage",
                    "id": "public-synthetic-user-1",
                    "content": [
                        {
                            "type": "text",
                            "text": PROJECTION_MARKER,
                            "text_elements": [],
                        }
                    ],
                },
                "started_at_ms": 0,
                "completed_at_ms": 0,
            },
        ),
        (
            "event_msg",
            {
                "type": "task_complete",
                "turn_id": TURN_ID,
                "last_agent_message": None,
            },
        ),
    ]
    lines = [
        json.dumps(
            {
                "timestamp": "2026-10-22T00:00:00Z",
                "ordinal": ordinal,
                "type": kind,
                "payload": payload,
            },
            separators=(",", ":"),
        )
        for ordinal, (kind, payload) in enumerate(payloads)
    ]
    return ("\n".join(lines) + "\n").encode()


class HttpsOnlyRedirect(urllib.request.HTTPRedirectHandler):
    """Reject a non-HTTPS redirect before issuing the redirected request."""

    def redirect_request(
        self, req: Any, fp: Any, code: int, msg: str,
        headers: Any, newurl: str,
    ) -> Any:
        require(urllib.parse.urlsplit(newurl).scheme == "https",
                "NON_HTTPS_REDIRECT")
        return super().redirect_request(req, fp, code, msg, headers, newurl)


def fetch_pin(url: str, path: Path, size: int, digest: str) -> None:
    """Download one public pinned asset with exact bytes, hash and a deadline."""
    started = time.monotonic()
    seen = 0
    hasher = hashlib.sha256()
    request = urllib.request.Request(url, headers={"User-Agent": "ANFL-public-CI"})
    opener = urllib.request.build_opener(HttpsOnlyRedirect())
    with opener.open(request, timeout=30) as response:
        require(response.url.startswith("https://"), "DOWNLOAD_REDIRECT_SCHEME")
        with path.open("xb") as target:
            while chunk := response.read(min(65_536, size + 1 - seen)):
                seen += len(chunk)
                require(seen <= size, "DOWNLOAD_SIZE_LIMIT")
                require(time.monotonic() - started <= 150, "DOWNLOAD_DEADLINE")
                hasher.update(chunk)
                target.write(chunk)
    require(seen == size, "DOWNLOAD_LENGTH")
    require(hasher.hexdigest() == digest, "DOWNLOAD_SHA256")
    emit("PUBLIC_DOWNLOAD_VERIFIED", bytes=seen, sha256=digest)


def extract_binary(archive: Path, target: Path) -> dict[str, Any]:
    """Extract exactly one regular bounded binary without trusting tar paths."""
    with tarfile.open(archive, "r:gz") as source:
        members = source.getmembers()
        require(len(members) == 1, "RELEASE_TAR_MEMBER_COUNT")
        member = members[0]
        require(member.isfile() and not member.issym(), "RELEASE_TAR_TYPE")
        require(
            member.name == "codex-aarch64-apple-darwin",
            "RELEASE_TAR_NAME",
        )
        require(0 < member.size <= 300_000_000, "RELEASE_BINARY_SIZE")
        stream = source.extractfile(member)
        require(stream is not None, "RELEASE_TAR_STREAM")
        hasher = hashlib.sha256()
        count = 0
        with stream, target.open("xb") as output:
            while chunk := stream.read(65_536):
                count += len(chunk)
                require(count <= member.size, "RELEASE_BINARY_CAP")
                hasher.update(chunk)
                output.write(chunk)
        require(count == member.size, "RELEASE_BINARY_EOF")
    target.chmod(0o700)
    return {"bytes": count, "sha256": hasher.hexdigest()}


def safe_environment() -> dict[str, str]:
    """Pass a small environment allowlist, preserving the runner's real home."""
    keys = ("PATH", "HOME", "USER", "LOGNAME", "TMPDIR", "LANG", "LC_ALL")
    return {key: os.environ[key] for key in keys if key in os.environ}


def sandbox_profile(root: Path, codex_root: Path) -> str:
    """Deny all native-child networking and writes outside disposable state."""
    paths = " ".join(
        f"(subpath {json.dumps(str(path))})" for path in (root, codex_root)
    )
    return (
        "(version 1)\n"
        "(allow default)\n"
        "(deny network*)\n"
        "(deny file-write*)\n"
        f"(allow file-write* {paths})\n"
        '(allow file-write-data (literal "/dev/null"))\n'
    )


def verify_network_denial(
    profile: Path, env: dict[str, str], root: Path
) -> None:
    """Verify TCP/UDP bind/connect denial and disposable SQLite writability."""
    code = (
        "import json,socket,sys\n"
        "out=[]\n"
        "for kind,op in [(socket.SOCK_STREAM,'bind'),"
        "(socket.SOCK_DGRAM,'bind'),"
        "(socket.SOCK_STREAM,'connect'),(socket.SOCK_DGRAM,'send')]:\n"
        " s=None\n"
        " try:\n"
        "  s=socket.socket(socket.AF_INET,kind)\n"
        "  if op=='bind':s.bind(('127.0.0.1',0))\n"
        "  elif op=='connect':s.connect(('127.0.0.1',int(sys.argv[1])))\n"
        "  else:s.sendto(b'PUBLIC',('127.0.0.1',int(sys.argv[2])))\n"
        "  out.append(0)\n"
        " except OSError as e:out.append(e.errno)\n"
        " finally:\n"
        "  if s is not None:s.close()\n"
        "print(json.dumps(out))\n"
    )
    with socket.socket() as listener, socket.socket(
        socket.AF_INET, socket.SOCK_DGRAM
    ) as datagram:
        listener.bind(("127.0.0.1", 0))
        listener.listen(2)
        datagram.bind(("127.0.0.1", 0))
        ports = [str(listener.getsockname()[1]), str(datagram.getsockname()[1])]
        control = subprocess.run(
            [sys.executable, "-I", "-B", "-c", code] + ports,
            env=env, capture_output=True, timeout=10, check=True,
        )
        require(json.loads(control.stdout) == [0] * 4, "NETWORK_CONTROL_FAILED")
        denied = subprocess.run(
            ["/usr/bin/sandbox-exec", "-f", str(profile), sys.executable,
             "-I", "-B", "-c", code] + ports,
            env=env, capture_output=True, timeout=10, check=True,
        )
        require(json.loads(denied.stdout) == [1] * 4, "NETWORK_DENIAL_UNPROVED")
    sqlite_code = (
        "import sqlite3,sys\n"
        "c=sqlite3.connect(sys.argv[1]);c.execute('PRAGMA journal_mode=WAL');"
        "c.execute('CREATE TABLE public_canary(n INTEGER)');"
        "c.execute('INSERT INTO public_canary VALUES(1)');c.commit();"
        "assert c.execute('SELECT n FROM public_canary').fetchone()==(1,);"
        "c.close()\n"
    )
    subprocess.run(
        ["/usr/bin/sandbox-exec", "-f", str(profile), sys.executable,
         "-I", "-B", "-c", sqlite_code, str(root / "sqlite" / "canary.db")],
        env=env, capture_output=True, timeout=10, check=True,
    )
    emit("REMOTE_ENVELOPE_CANARIES_VERIFIED",
         tcp_udp_bind_connect_errno=[1] * 4, sqlite_wal_write=True,
         ipv6_empirically_tested=False, policy="deny network*")




class NativeClient:
    """Own one bounded stdio native process and positively observe its closure."""

    def __init__(
        self, command: list[str], env: dict[str, str], cwd: Path
    ) -> None:
        self.selector = selectors.DefaultSelector()
        self.buffer = b""
        self.stderr = bytearray()
        self.total = 0
        self.messages = 0
        self.sequence = 0
        self.closed = False
        self.methods: list[str] = []
        try:
            self.process = subprocess.Popen(
                command,
                stdin=subprocess.PIPE,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                env=env,
                cwd=cwd,
                start_new_session=True,
                bufsize=0,
            )
        except Exception:
            self.selector.close()
            raise
        try:
            require(self.process.stdout is not None, "MISSING_STDOUT")
            require(self.process.stderr is not None, "MISSING_STDERR")
            for stream, label in (
                (self.process.stdout, "stdout"),
                (self.process.stderr, "stderr"),
            ):
                os.set_blocking(stream.fileno(), False)
                self.selector.register(stream, selectors.EVENT_READ, label)
        except Exception:
            self.close()
            raise

    def send(self, message: dict[str, Any]) -> None:
        """Write one small synthetic protocol message."""
        encoded = json.dumps(message, separators=(",", ":")).encode() + b"\n"
        require(len(encoded) <= 16_384, "REQUEST_SIZE")
        require(self.process.stdin is not None, "MISSING_STDIN")
        self.process.stdin.write(encoded)
        self.process.stdin.flush()

    def pump(self, timeout: float) -> list[dict[str, Any]]:
        """Read bounded pipes without allowing shutdown capture to grow freely."""
        replies: list[dict[str, Any]] = []
        for key, _ in self.selector.select(timeout=timeout):
            chunk = os.read(key.fileobj.fileno(), 65_536)
            if not chunk:
                self.selector.unregister(key.fileobj)
                continue
            self.total += len(chunk)
            require(self.total <= MAX_CAPTURE, "CAPTURE_CAP")
            if key.data == "stderr":
                self.stderr.extend(chunk)
                require(len(self.stderr) <= 262_144, "STDERR_CAP")
                continue
            self.buffer += chunk
            while b"\n" in self.buffer:
                line, self.buffer = self.buffer.split(b"\n", 1)
                require(len(line) <= MAX_LINE, "STDOUT_LINE_CAP")
                self.messages += 1
                require(self.messages <= 512, "MESSAGE_CAP")
                message = json.loads(line)
                require(isinstance(message, dict), "PROTOCOL_OBJECT")
                require(
                    not ("method" in message and "id" in message),
                    "UNEXPECTED_SERVER_REQUEST",
                )
                replies.append(message)
            require(len(self.buffer) <= MAX_LINE, "PARTIAL_STDOUT_LINE_CAP")
        return replies

    def rpc(
        self, method: str, params: dict[str, Any], timeout: float = 45
    ) -> dict[str, Any]:
        """Wait only for the matching response, enforcing stream/lifecycle caps."""
        self.methods.append(method)
        self.sequence += 1
        request_id = self.sequence
        self.send({"jsonrpc": "2.0", "id": request_id, "method": method,
                   "params": params})
        deadline = time.monotonic() + timeout
        while time.monotonic() < deadline:
            match: dict[str, Any] | None = None
            for message in self.pump(timeout=0.1):
                if "id" in message:
                    require(message["id"] == request_id, "UNEXPECTED_REPLY_ID")
                    require(match is None, "DUPLICATE_REPLY")
                    match = message
            if match is not None:
                return match
            require(self.process.poll() is None, "NATIVE_EARLY_EXIT")
        raise RuntimeError(f"RPC_DEADLINE:{method}")

    def close(self) -> dict[str, Any]:
        """Drain with caps and reap only this owned process and process group."""
        if self.closed:
            return {"already_closed": True}
        self.closed = True
        forced = False
        errors: list[str] = []

        def note(error: BaseException) -> None:
            errors.append(f"{type(error).__name__}:{error}")

        def signal_group(kind: signal.Signals) -> None:
            try:
                os.killpg(self.process.pid, kind)
            except ProcessLookupError:
                pass
            except OSError as error:
                note(error)

        try:
            require(self.process.stdin is not None, "MISSING_STDIN_AT_CLOSE")
            self.process.stdin.close()
            deadline = time.monotonic() + 15
            while self.selector.get_map() and time.monotonic() < deadline:
                self.pump(timeout=0.1)
            self.process.wait(timeout=max(0.1, deadline - time.monotonic()))
            require(not self.selector.get_map(), "PIPE_EOF_NOT_OBSERVED")
            require(not self.buffer, "PARTIAL_PROTOCOL_AT_EOF")
        except Exception as error:
            forced = True
            note(error)
        if forced or self.process.poll() is None:
            forced = True
            signal_group(signal.SIGTERM)
            try:
                self.process.wait(timeout=5)
            except subprocess.TimeoutExpired:
                signal_group(signal.SIGKILL)
                try:
                    self.process.wait(timeout=5)
                except Exception as error:
                    note(error)
            except Exception as error:
                note(error)
        try:
            self.selector.close()
        except Exception as error:
            forced = True
            note(error)
        for stream in (
            self.process.stdin, self.process.stdout, self.process.stderr
        ):
            if stream is not None:
                try:
                    stream.close()
                except Exception as error:
                    forced = True
                    note(error)
        group_gone = False
        for attempt in range(11):
            try:
                os.killpg(self.process.pid, 0)
            except ProcessLookupError:
                group_gone = True
                break
            except OSError as error:
                note(error)
                break
            forced = True
            if attempt == 0:
                signal_group(signal.SIGKILL)
            time.sleep(0.1)
        result = {
            "exit": self.process.returncode,
            "forced": forced,
            "group_gone": group_gone,
            "errors": errors,
            "captured_bytes": self.total,
            "messages": self.messages,
            "requested_methods": self.methods,
            "stderr_tail": bytes(self.stderr[-8192:]).decode(errors="replace"),
        }
        emit("NATIVE_PROCESS_CLOSED", **result)
        return result


def config_args(root: Path, sqlite_root: Path, catalog: Path) -> list[str]:
    """Return reviewed supported startup overrides for the fresh runner."""
    provider = {
        "name": "ANFL public offline synthetic",
        "base_url": "http://127.0.0.1:1/v1",
        "wire_api": "responses",
        "requires_openai_auth": False,
        "supports_websockets": False,
        "request_max_retries": 0,
        "stream_max_retries": 0,
    }
    model = json.loads(catalog.read_text())["models"][0]["slug"]
    overrides: dict[str, Any] = {
        "sqlite_home": str(sqlite_root),
        "model_catalog_json": str(catalog),
        "model": model,
        "model_provider": "anfl_public_offline",
        "model_providers.anfl_public_offline": provider,
        "cli_auth_credentials_store": "file",
        "approval_policy": "never",
        "sandbox_mode": "read-only",
        "features.hooks": False,
        "features.plugins": False,
        "features.shell_snapshot": False,
        "features.shell_snapshot_v2": False,
        "features.code_mode_prewarm": False,
        "features.skip_host_skill_discovery": True,
        "skills.bundled.enabled": False,
        "skills.include_instructions": False,
        "cloud.skills.enabled": False,
        "orchestrator.mcp.enabled": False,
    }
    args: list[str] = []
    for key, value in overrides.items():
        if isinstance(value, dict):
            encoded = "{ " + ", ".join(
                f"{name} = {json.dumps(item)}" for name, item in value.items()
            ) + " }"
        else:
            encoded = json.dumps(value)
        args.extend(["-c", f"{key}={encoded}"])
    return args


def initialize(client: NativeClient) -> None:
    """Initialize the experimental-capable synthetic client exactly once."""
    reply = client.rpc("initialize", {
        "clientInfo": {"name": "anfl_public_synthetic", "version": "1.0"},
        "capabilities": {"experimentalApi": True},
    }, timeout=15)
    require("result" in reply, "INITIALIZE_FAILED")
    agent = reply["result"].get("userAgent", "")
    require("0.163.0" in agent, "BACKEND_VERSION_ASSERTION")
    client.send({"jsonrpc": "2.0", "method": "initialized"})
    emit("INITIALIZE_OK", user_agent=agent)


def resume(
    client: NativeClient, expected: bool, path: Path, cwd: Path, model: str
) -> dict[str, Any]:
    """Resume the synthetic identity, never an actual recovery identity."""
    reply = client.rpc("thread/resume", {
        "threadId": THREAD_ID, "excludeTurns": True,
    }, timeout=30)
    if not expected:
        require("error" in reply, "ABSENT_SOURCE_UNEXPECTED_SUCCESS")
        error = reply["error"]
        require(error.get("code") == -32600, "ABSENCE_WRONG_ERROR_CODE")
        require(error.get("message") ==
                f"no rollout found for thread id {THREAD_ID}",
                "ABSENCE_WRONG_ERROR_REASON")
        emit("ABSENT_SOURCE_REJECTED", error=reply["error"])
        return reply
    require("result" in reply, "RESUME_FAILED")
    require(reply["result"].get("modelProvider") == "anfl_public_offline",
            "RESUME_WRONG_PROVIDER")
    require(reply["result"].get("model") == model, "RESUME_WRONG_MODEL")
    thread = reply["result"]["thread"]
    require(thread["id"] == THREAD_ID, "RESUME_WRONG_ID")
    require(thread.get("historyMode") == "paginated", "RESUME_WRONG_MODE")
    require(thread.get("path") == str(path), "RESUME_WRONG_PATH")
    require(thread.get("cwd") == str(cwd), "RESUME_WRONG_CWD")
    require(thread.get("source") == "cli", "RESUME_WRONG_SOURCE")
    require(thread.get("status") == {"type": "idle"}, "RESUME_NOT_IDLE")
    emit("RESUME_OK", thread_id=thread["id"], history_mode=thread["historyMode"],
         path=thread["path"], cwd=thread["cwd"])
    return reply


def verify_marker(client: NativeClient) -> None:
    """Verify supported projected-history readback, not checkpoint equivalence."""
    turns = client.rpc("thread/turns/list", {"threadId": THREAD_ID, "limit": 2,
                                               "sortDirection": "asc"})
    require("result" in turns, "TURNS_LIST_FAILED")
    require(
        turns["result"].get("nextCursor") is None, "UNEXPECTED_TURN_PAGINATION"
    )
    turn_rows = turns["result"]["data"]
    require(len(turn_rows) == 1, "PROJECTED_TURN_COUNT")
    require(turn_rows[0]["id"] == TURN_ID, "PROJECTED_TURN_ID")
    require(turn_rows[0]["status"] == "completed", "PROJECTED_TURN_STATUS")
    items = client.rpc("thread/items/list", {
        "threadId": THREAD_ID, "turnId": TURN_ID, "limit": 2,
        "sortDirection": "asc",
    })
    require("result" in items, "ITEMS_LIST_FAILED")
    require(
        items["result"].get("nextCursor") is None, "UNEXPECTED_ITEM_PAGINATION"
    )
    item_rows = items["result"]["data"]
    require(len(item_rows) == 1, "PROJECTED_ITEM_COUNT")
    require(item_rows[0]["turnId"] == TURN_ID, "PROJECTED_ITEM_TURN")
    item = item_rows[0]["item"]
    require(item["type"] == "userMessage", "PROJECTED_ITEM_TYPE")
    require(item["id"] == "public-synthetic-user-1", "PROJECTED_ITEM_ID")
    require(
        item["content"][0]["text"] == PROJECTION_MARKER,
        "PROJECTED_MARKER_MISSING",
    )
    require(
        REPLACEMENT_MARKER not in json.dumps(items["result"]),
        "REPLACEMENT_PROJECTION_ASSUMPTION",
    )
    emit("PROJECTED_MARKER_OK", replacement_equivalence_proven=False)


def verify_source(path: Path, original: bytes) -> dict[str, Any]:
    """Check this synthetic native file's prefix, append and ordinal chain."""
    require(path.stat().st_size <= 1_048_576, "NATIVE_FIXTURE_SIZE_CAP")
    current = path.read_bytes()
    require(current.startswith(original), "FIXTURE_PREFIX_CHANGED")
    require(current.endswith(b"\n"), "NATIVE_FIXTURE_FINAL_LF")
    rows = [json.loads(line) for line in current.splitlines()]
    require(5 < len(rows) <= 1024, "NATIVE_APPEND_CENSUS")
    require(
        [row.get("ordinal") for row in rows] == list(range(len(rows))),
        "NATIVE_ORDINAL_CHAIN",
    )
    return {
        "bytes": len(current),
        "records": len(rows),
        "sha256": hashlib.sha256(current).hexdigest(),
        "original_prefix_preserved": True,
        "native_appended_records": len(rows) - 5,
        "contiguous_ordinals": True,
    }


def run_remote() -> None:
    """Execute the exact public-only, remote-only, bounded qualification."""
    require(os.environ.get("GITHUB_ACTIONS") == "true", "REMOTE_CI_ONLY")
    require(os.environ.get("GITHUB_REPOSITORY") ==
            "MattSStack/anfl-public-validation", "PUBLIC_REPOSITORY_ONLY")
    require(platform.system() == "Darwin", "MACOS_ONLY")
    require(platform.machine() == "arm64", "ARM64_ONLY")
    require(sys.version_info >= (3, 11), "PYTHON_VERSION")
    require("CODEX_HOME" not in os.environ, "INHERITED_CODEX_HOME")
    require("CODEX_SQLITE_HOME" not in os.environ, "INHERITED_SQLITE_HOME")
    codex_root = Path(os.environ["HOME"]) / ".codex"
    require(not codex_root.exists(), "RUNNER_CODEX_STATE_NOT_FRESH")
    require(not Path("/etc/codex").exists(), "SYSTEM_CODEX_CONFIG_EXISTS")
    root = Path(tempfile.mkdtemp(
        prefix="anfl-public-native-", dir=os.environ["RUNNER_TEMP"]
    )).resolve()
    root.chmod(0o700)
    codex_root.mkdir(mode=0o700)
    sqlite_root = root / "sqlite"
    sqlite_root.mkdir(mode=0o700)
    workspace = root / "workspace"
    workspace.mkdir(mode=0o700)
    archive = root / "codex.tar.gz"
    catalog = root / "models.json"
    binary = root / "codex"
    fetch_pin(ASSET_URL, archive, ASSET_BYTES, ASSET_SHA)
    fetch_pin(CATALOG_URL, catalog, CATALOG_BYTES, CATALOG_SHA)
    binary_pin = extract_binary(archive, binary)
    profile = root / "native.sb"
    profile.write_text(sandbox_profile(root, codex_root))
    env = safe_environment()
    child_temp = root / "child-tmp"
    child_temp.mkdir(mode=0o700)
    env["TMPDIR"] = str(child_temp)
    verify_network_denial(profile, env, root)
    envelope = ["/usr/bin/sandbox-exec", "-f", str(profile), str(binary)]
    version = subprocess.run(
        envelope + ["--version"], env=env, cwd=workspace,
        capture_output=True, timeout=15, check=True,
    )
    require(version.stdout.strip() == b"codex-cli 0.163.0", "CLI_VERSION")
    emit("RELEASE_BINARY_VERIFIED", **binary_pin)
    model = json.loads(catalog.read_text())["models"][0]["slug"]
    command = envelope + config_args(
        root, sqlite_root, catalog
    ) + ["app-server", "--strict-config", "--listen", "stdio://"]
    managed = codex_root / "sessions" / "2026" / "10" / "22"
    managed.mkdir(parents=True, mode=0o700)
    path = managed / f"rollout-2026-10-22T00-00-00-{THREAD_ID}.jsonl"
    held = root / "synthetic-source-held.jsonl"
    original = fixture_bytes(str(workspace))
    source_pin = hashlib.sha256(original).hexdigest()
    closures: list[dict[str, Any]] = []
    for phase in ("first", "missing_after_close", "cold_reopen"):
        phase_error: Exception | None = None
        client = NativeClient(command, env, workspace)
        try:
            initialize(client)
            if phase == "first":
                resume(client, expected=False, path=path, cwd=workspace,
                       model=model)
                path.write_bytes(original)
                path.chmod(0o600)
                resume(client, expected=True, path=path, cwd=workspace,
                       model=model)
                verify_marker(client)
            elif phase == "missing_after_close":
                resume(client, expected=False, path=path, cwd=workspace,
                       model=model)
            else:
                resume(client, expected=True, path=path, cwd=workspace,
                       model=model)
                verify_marker(client)
        except Exception as error:
            phase_error = error
        finally:
            closure = client.close()
            closures.append(closure)
        if phase_error is not None:
            raise phase_error
        require(not closure["forced"], "FORCED_SHUTDOWN_NOT_QUALIFIED")
        require(closure["exit"] == 0, "NATIVE_EXIT_NOT_CLEAN")
        require(closure["group_gone"], "NATIVE_PROCESS_GROUP_STILL_PRESENT")
        if phase == "first":
            emit("NATIVE_APPEND_VERIFIED", **verify_source(path, original))
            path.rename(held)
        elif phase == "missing_after_close":
            require(not path.exists(), "MISSING_SOURCE_RECREATED")
            emit("HELD_SOURCE_PRESERVED", **verify_source(held, original))
            held.rename(path)
    final_source = verify_source(path, original)
    emit("COLD_SOURCE_PREFIX_AND_ORDINALS_VERIFIED", **final_source)
    emit(
        "SYNTHETIC_NATIVE_CONSUMER_QUALIFIED",
        release="0.163.0",
        upstream_commit=REVISION,
        fixture_sha256=source_pin,
        original_fixture_bytes=len(original),
        current_fixture_bytes=final_source["bytes"],
        native_appended_records=final_source["native_appended_records"],
        process_count=len(closures),
        clean_closures=all(row["exit"] == 0 and not row["forced"]
                           for row in closures),
        requested_model_turns=sum(
            method == "turn/start"
            for row in closures for method in row["requested_methods"]
        ),
        native_network="OS_DENIED",
        input_contract="public synthetic fixture only on a fresh remote runner",
        replacement_history_equivalence_proven=False,
        real_archive_restoration_proven=False,
        independent_custody_proven=False,
        retirement_authorized=False,
    )


def self_test() -> None:
    """Validate fixture invariants and fail-closed guards without runtime I/O."""
    raw = fixture_bytes("/public/synthetic")
    rows = [json.loads(line) for line in raw.splitlines()]
    require(len(rows) == 5 and raw.endswith(b"\n"), "TEST_RECORD_CENSUS")
    require([row["ordinal"] for row in rows] == list(range(5)), "TEST_ORDINALS")
    require(rows[0]["payload"]["history_base"] is None, "TEST_LINEAGE")
    require(rows[0]["payload"]["id"] == THREAD_ID, "TEST_ID")
    require(rows[0]["payload"]["history_mode"] == "paginated", "TEST_MODE")
    require(rows[1]["payload"]["window_number"] == 1, "TEST_WINDOW")
    require(REPLACEMENT_MARKER.encode() in raw, "TEST_REPLACEMENT")
    require(PROJECTION_MARKER.encode() in raw, "TEST_PROJECTION")
    require("turn/start" not in raw.decode(), "TEST_NO_TURN")
    require(len(raw) < 4096, "TEST_FIXTURE_CAP")
    profile = sandbox_profile(Path("/remote/root"), Path("/remote/codex"))
    require("(deny network*)" in profile, "TEST_NETWORK_PROFILE")
    require("(deny file-write*)" in profile, "TEST_WRITE_PROFILE")
    try:
        require(False, "EXPECTED_GUARD")
    except RuntimeError as error:
        require(str(error) == "EXPECTED_GUARD", "TEST_GUARD_IDENTITY")
    else:
        raise RuntimeError("TEST_GUARD_NOT_FAIL_CLOSED")
    emit("FIXTURE_CONTRACT_CHECKS_OK", assertions=13,
         native_processes=0, network_requests=0)


def main() -> int:
    """Dispatch explicitly selected self-test or remote qualification."""
    parser = argparse.ArgumentParser()
    parser.add_argument("--self-test", action="store_true")
    options = parser.parse_args()
    try:
        if options.self_test:
            self_test()
        else:
            run_remote()
    except Exception as error:
        emit("ERROR", exception=type(error).__name__, detail=str(error))
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
```

### Full workflow

```yaml
# ANFL | The Reactor™ | Layer 01 | Version 1.0 | 2026-10-22
name: ANFL public synthetic native consumer
run-name: Synthetic native consumer ${{github.run_number}}
on:
  workflow_dispatch:
permissions:
  contents: read
concurrency:
  group: anfl-public-native-consumer
  cancel-in-progress: false
jobs:
  qualify:
    if: >-
      github.repository == 'MattSStack/anfl-public-validation' &&
      github.event.repository.private == false
    runs-on: macos-15
    timeout-minutes: 12
    steps:
      - name: Fetch pinned public helper and qualify synthetic native consumption
        shell: python
        env:
          ANFL_PROBE_REVISION: ${{github.sha}}
          ANFL_PROBE_SHA256: 05a7b1775e91bc889053564a06336119ec43e55b86c1bc696655b9dd91e4981b
        run: |
          import hashlib
          import os
          import pathlib
          import subprocess
          import sys
          import tempfile
          import urllib.request

          path = (
              "06_api_integration/"
              "261022_PY_NATIVE_CONSUMER_PROBE_INT_v1.0_ANFL.py"
          )
          url = (
              "https://raw.githubusercontent.com/"
              "MattSStack/anfl-public-validation/"
              + os.environ["ANFL_PROBE_REVISION"] + "/" + path
          )
          with urllib.request.urlopen(url, timeout=30) as response:
              data = response.read(65537)
          if len(data) > 65536:
              raise RuntimeError("PUBLIC_HELPER_SIZE_LIMIT")
          digest = hashlib.sha256(data).hexdigest()
          if digest != os.environ["ANFL_PROBE_SHA256"]:
              raise RuntimeError("PUBLIC_HELPER_SHA256")
          root = pathlib.Path(tempfile.mkdtemp(
              prefix="anfl-probe-helper-", dir=os.environ["RUNNER_TEMP"]
          ))
          helper = root / pathlib.Path(path).name
          helper.write_bytes(data)
          subprocess.run(
              [sys.executable, "-B", str(helper), "--self-test"],
              check=True, timeout=20,
          )
          subprocess.run(
              [sys.executable, "-B", str(helper)], check=True, timeout=680
          )
```

## Ops / Runbook

Dispatch manually once only after review. Record commit, workflow ID, run ID,
attempt and job ID before monitoring. Monitor only that owned run. On failure or
unknown outcome, retain the primary error and lifecycle evidence, reconcile
before any retry, and do not broaden the network or filesystem envelope.

Standard GitHub-hosted runners for public repositories are documented free.
This workflow creates no billable external service or upload.
[GitHub Actions free use](https://docs.github.com/en/billing/concepts/product-billing/github-actions#free-use-of-github-actions).

Rollback: leave the manual workflow undispatched, or cancel only its exact owned
run. Revert the additive public commit if necessary. Keep prior qualified native
scanner artifacts and recovery-owner evidence intact. Runner state is disposable;
no recovery source or original history is moved by this test.
