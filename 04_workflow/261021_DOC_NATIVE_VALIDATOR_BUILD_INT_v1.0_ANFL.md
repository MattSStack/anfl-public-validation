# The Reactor™ native validator build contract

Created: 2026-10-22T05:47:00Z
Updated: 2026-10-22T06:27:23Z
Version: 1.0
Classification: B
Layers: 01 infrastructure / 06 API integration

## BLUF

Build the bounded stdin validator against the exact official OpenAI Codex source commit on an isolated standard public GitHub macOS ARM64 runner. The first run stopped before compilation because the pinned upstream lockfile contains stale local release-version stamps. The revised pipeline accepts only manifest-proven local version stamps, through a strict audited proposal, before running all native compilation with --locked.

## Executive

The existing validator adds one auto-discovered binary source to the public codex-rollout package and introduces no dependency. A separate standard-library Python auditor compares the original and resolver-proposed lockfiles, discovers the actual public workspace manifests, and refuses every external package change, dependency-list change, local roster change, metadata change, or unproved local version. It accepts local 0.0.0 to 0.163.0 only when backed by that package's actual manifest.

This repository is an additive public build mirror. It receives only generated public code, public upstream source and synthetic inputs. All recovery payloads, operator credentials, private histories and keys remain outside it.

## Implementation Status

```yaml
component: "The Reactor native validator remote build"
status: "in_progress"
version: "1.0"
position_classification: "B"
layer: "06"
progress_percentage: 65
dependencies:
  - "Actual remote manifest walk and strict lock audit"
  - "32 native Rust tests and bounded synthetic CLI checks"
  - "Verified draft release assets and manifest"
```

## Architecture

```text
Pinned public helper + auditor + upstream commit
                       |
     resolver-only lock proposal (no native compile)
                       |
 strict manifest-backed audit + resolved-lock SHA
                       |
        locked native tests → locked build
                       |
   unchanged lock + synthetic CLI + ARM64 checks
                       |
       bounded draft release + build manifest
```

Upstream commit: 83086188ae0614bd882818ff1aa23f3194962012.
Upstream original Cargo.lock SHA256: 17cdd862471774ba7aef836aaf0a0a3ed76bd23e503b1fc7278c89d1f2719d03.
Rust: 1.95.0. Target: aarch64-apple-darwin. Runner: standard macos-15, ARM64.
Helper: 30466 bytes, SHA256 65c4f7a10788052e94a5fa16a12d93d419fcbddfc27ecee6fa3577957f065ddf.
Auditor: 26532 bytes, SHA256 c8a14e151547f4ce81aff8501f7e874ad3a5dc9b753a0af1691555b1648c5e1b.
Workflow: 11961 bytes, SHA256 dbdf77b79f806b94bf6465891810d935bf4815bfcdccee251dbf5ae5166177c6.
The Rust successor changes a header comment only; its executable body matches the independently reviewed source.

## Implementation

The validator reads stdin only. Limits are 8 MiB total, 4 MiB per record, 10,000 records, 8,192-byte scratch reads, and 4,096-byte fixed-schema output. It requires final LF and EOF, stops at the first native decode error, suppresses raw parser/panic payloads, and treats output errors as failure. It invokes the official parse_rollout_line_bytes decoder rather than a substitute schema. Typed variant and compaction counts do not prove lossless recovery, lineage or coverage. Byte limits are not a hard process heap or wall-time limit.

The auditor preserves strict TOML types, exact Boolean workspace inheritance, complete external package dictionaries, dependency qualifiers and ordering. It uses bounded no-follow regular-file reads, descriptor closure accounting, nonblocking leaf opens and memoized glob matching. Its 24 passing memory-only tests qualify comparison and path policy; they do not qualify the entire filesystem walker. The CI step must exercise that walker on the pinned public checkout.

The workflow validates the original lock pin, creates a resolver-only proposal with cargo tree, audits it, and immediately binds the actual lock bytes to the auditor's resolved SHA. Native tests and build both use --locked. Post-build comparison requires the same resolved lock bytes. The git diff restriction covers tracked changes; the injected helper source is an intentional untracked input. Initial source hashing supplies its provenance, without claiming the diff check proves all untracked paths unchanged.

### Validation and current evidence

- Rust executable source: independent static review passed; 32 native tests authored, not yet observed running.
- Auditor: corrected pin independently approved; 24/24 memory-only tests passed, zero filesystem fixtures.
- Workflow: revised integration review passed; two zsh blocks and two embedded Python blocks parsed locally.
- First CI run 39730551641 at commit 6edf0eff78ab25e25acb6b8c74ea207b57d9bedd failed before compilation on --locked at 2026-10-22T05:53:38Z. No native tests ran and release publication was skipped.
- Revised run: pending. No usable native artifact is claimed at this stage.

### Threat notes

The public runner downloads exact generated source pins and the exact public upstream commit. Build steps receive no GH_TOKEN or operator secrets. The job has contents:write capability because draft release upload requires it; that permission is broad within this isolated repository, rather than release-only. GH_TOKEN is explicitly provided only to the publication step. There is no attempt to inspect or change local credentials.

The job-level public-repository guard executes before runner allocation. Publication additionally checks that the current default-branch head still matches the build commit. Draft assets are not a public distribution release, and third-party notice review remains necessary before public binary distribution. Release creation and uploads are not atomic: reconcile partial draft assets after a publication failure; do not assume no remote write occurred.

### Cost and environment

Standard GitHub-hosted runner compute in public repositories is documented as free. This workflow uses no larger runner, Actions artifact upload, cache, Packages, LFS, custom image or paid cloud resource. Release delivery cost is a qualified inference from current no-quota release documentation plus a historical GitHub Staff no-financial-impact answer; a current explicit zero-dollar price guarantee was not found. Assets are capped at one binary of at most 64 MiB, one manifest of at most 64 KiB, and a checksum file.

No Rust toolchain, native build, bulk source checkout or binary download is performed on the local Mac. New durable source and receipts stay on the authorized external workspace.

## Ops / Runbook

1. Publish the exact reviewed public source, auditor, workflow and contract in one non-forced commit. Mirror the canonical workflow blob into .github/workflows using the same Git blob SHA.
2. Record dispatch intent before one workflow_dispatch. Capture commit, run and job IDs; an uncertain response is reconciled against remote state before any retry.
3. Observe only that owned CI job. On an audit or compilation failure, inspect bounded public logs, preserve the failed run and correct the demonstrated cause.
4. On success, verify all 32 native tests, the actual lock audit, ARM64/toolchain identity, source/lock/binary pins and draft asset count/size.
5. Do not run private inputs or treat this helper build as authorization to access recovery sources. Any later candidate qualification remains separately controlled by the recovery Owner.

### Rollback

Before branch update, staged Git blobs have no effect on the active workflow. After update, use a normal reviewed revert commit; do not force-push. Cancel only an explicitly identified owned CI run when necessary. Reconcile and remove only its identified partial draft release if rollback requires it; never apply broad cleanup. Preserve the append-only run accounting and failure evidence.

### Sources

- [Exact official native decoder source](https://github.com/openai/codex/blob/83086188ae0614bd882818ff1aa23f3194962012/codex-rs/rollout/src/lib.rs)
- [Exact official workspace manifest](https://github.com/openai/codex/blob/83086188ae0614bd882818ff1aa23f3194962012/codex-rs/Cargo.toml)
- [Standard public GitHub runner specifications](https://docs.github.com/en/actions/reference/runners/github-hosted-runners#standard-github-hosted-runners-for-public-repositories)
- [Free use of GitHub Actions](https://docs.github.com/en/billing/concepts/product-billing/github-actions#free-use-of-github-actions)
- [Release storage and bandwidth quotas](https://docs.github.com/en/repositories/releasing-projects-on-github/about-releases#storage-and-bandwidth-quotas)
- [Historical GitHub Staff cost answer](https://github.com/orgs/community/discussions/127096)
- [Workflow permissions](https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax#permissions)

## Change Set and complete source snapshots

1. Native Rust stdin validator.
2. Strict Python lock auditor.
3. Canonical workflow; its .github/workflows copy must use the identical Git blob.
4. Build contract and local run accounting.

Each code fence below contains the complete canonical source at the stated pin. The LF immediately before a closing fence belongs to the source when the source already ends in LF; no additional presentation-only blank line is inserted.

### 06_api_integration/261021_RUST_NATIVE_ROLLOUT_VALIDATOR_INT_v1.0_ANFL.rs

````rust
/*
File: 261021_RUST_NATIVE_ROLLOUT_VALIDATOR_INT_v1.0_ANFL.rs
Product: The Reactor(TM)
Component: Native typed rollout stdin validator
Version: 1.0
Layer: 06_api_integration
Implementation status: source implemented; compilation and tests NOT yet run.
Upstream: openai/codex@83086188ae0614bd882818ff1aa23f3194962012
Toolchain: official codex-rs/rust-toolchain.toml pins Rust 1.95.0.
Integration: copy this PUBLIC source into the pinned workspace at
codex-rs/rollout/src/bin/anfl-rollout-stdin-validator.rs. The published
codex-rollout manifest autodiscovers the binary without new dependencies.
CI separately audits the upstream release-version lock stamps before building.
Run only separately authorized public-source CI builds.

BLUF: Decode each bounded LF-framed record with the official native parser.
No files, configuration, authentication, app-server, or rollout loader.
Typed compatibility is not lossless fidelity, lineage, compaction eligibility,
consumer independence, or recovery proof. Native decoding can allocate more
than the raw record; an external supervisor owns process memory/time limits.
Do not pass private recovery data to public CI.

Architecture: stdin -> bounded raw framing -> native typed record -> counters
-> drop record -> one bounded JSON report. Any invalid record stops the stream.
Runtime accepts no arguments and cannot select data-file paths.
*/
#![forbid(unsafe_code)]

use std::fmt::Write as _;
use std::io::{self, Read, Write};

use codex_rollout::{CompactedItem, RolloutItem};

const SCRATCH_BYTES: usize = 8_192;
const MAX_REPORT_BYTES: usize = 4_096;
const VARIANT_COUNT: usize = 12;
const STDOUT_FAILURE_EXIT: i32 = 3;
const VARIANT_NAMES: [&str; VARIANT_COUNT] = [
    "session_meta",
    "response_item",
    "inter_agent_communication",
    "inter_agent_communication_metadata",
    "compacted",
    "turn_context",
    "token_usage_record",
    "world_state",
    "security_risk_score",
    "retained_context",
    "event_msg",
    "realtime_item",
];

#[derive(Clone, Copy, Debug)]
struct Limits {
    aggregate_bytes: u64,
    record_bytes: usize,
    records: u64,
}

impl Limits {
    const PRODUCTION: Self = Self {
        aggregate_bytes: 8_388_608,
        record_bytes: 4_194_304,
        records: 10_000,
    };

    fn is_within_profile(self) -> bool {
        self.aggregate_bytes <= Self::PRODUCTION.aggregate_bytes
            && self.record_bytes <= Self::PRODUCTION.record_bytes
            && self.records <= Self::PRODUCTION.records
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Code {
    Ok,
    Arguments,
    Limits,
    EmptyInput,
    EmptyRecord,
    AggregateLimit,
    RecordLimit,
    RecordCountLimit,
    FinalLfRequired,
    NativeDecode,
    StdinIo,
    Allocation,
    CounterOverflow,
    Internal,
}

impl Code {
    fn label(self) -> &'static str {
        match self {
            Self::Ok => "OK",
            Self::Arguments => "ARGUMENTS",
            Self::Limits => "LIMITS",
            Self::EmptyInput => "EMPTY_INPUT",
            Self::EmptyRecord => "EMPTY_RECORD",
            Self::AggregateLimit => "AGGREGATE_LIMIT",
            Self::RecordLimit => "RECORD_LIMIT",
            Self::RecordCountLimit => "RECORD_COUNT_LIMIT",
            Self::FinalLfRequired => "FINAL_LF_REQUIRED",
            Self::NativeDecode => "NATIVE_DECODE",
            Self::StdinIo => "STDIN_IO",
            Self::Allocation => "ALLOCATION",
            Self::CounterOverflow => "COUNTER_OVERFLOW",
            Self::Internal => "INTERNAL",
        }
    }

    fn exit_code(self) -> i32 {
        match self {
            Self::Ok => 0,
            Self::StdinIo => 3,
            Self::Allocation | Self::CounterOverflow | Self::Internal => 4,
            _ => 2,
        }
    }
}

/// Counts describe decoded native variants, not raw wire-type occurrences.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Counts {
    bytes_read: u64,
    records: u64,
    max_record_bytes: u64,
    variants: [u64; VARIANT_COUNT],
    compacted_with_replacement: u64,
    replacement_items_total: u64,
    replacement_items_max: u64,
}

impl Counts {
    fn observe(
        &mut self,
        item: &RolloutItem,
        record_bytes: usize,
    ) -> Result<(), Code> {
        let mut next = *self;
        next.records = next.records.checked_add(1).ok_or(Code::CounterOverflow)?;
        let index = match item {
            RolloutItem::SessionMeta(_) => 0,
            RolloutItem::ResponseItem(_) => 1,
            RolloutItem::InterAgentCommunication(_) => 2,
            RolloutItem::InterAgentCommunicationMetadata { .. } => 3,
            RolloutItem::Compacted(compacted) => {
                count_compaction(&mut next, compacted)?;
                4
            }
            RolloutItem::TurnContext(_) => 5,
            RolloutItem::TokenUsageRecord(_) => 6,
            RolloutItem::WorldState(_) => 7,
            RolloutItem::SecurityRiskScore(_) => 8,
            RolloutItem::RetainedContext(_) => 9,
            RolloutItem::EventMsg(_) => 10,
            RolloutItem::RealtimeItem(_) => 11,
        };
        next.variants[index] = next.variants[index]
            .checked_add(1)
            .ok_or(Code::CounterOverflow)?;
        let length = u64::try_from(record_bytes).map_err(|_| Code::CounterOverflow)?;
        next.max_record_bytes = next.max_record_bytes.max(length);
        *self = next;
        Ok(())
    }
}

fn count_compaction(counts: &mut Counts, item: &CompactedItem) -> Result<(), Code> {
    if let Some(items) = item.replacement_history.as_ref() {
        let length = u64::try_from(items.len()).map_err(|_| Code::CounterOverflow)?;
        counts.compacted_with_replacement = counts
            .compacted_with_replacement
            .checked_add(1)
            .ok_or(Code::CounterOverflow)?;
        counts.replacement_items_total = counts
            .replacement_items_total
            .checked_add(length)
            .ok_or(Code::CounterOverflow)?;
        counts.replacement_items_max = counts.replacement_items_max.max(length);
    }
    Ok(())
}

#[derive(Debug)]
struct Failure {
    code: Code,
    counts: Counts,
}

fn failure(code: Code, counts: Counts) -> Failure {
    Failure { code, counts }
}

/// Validate stdin-like bytes without opening paths or retaining whole history.
///
/// Reads consume at most aggregate_cap + 1 bytes. Record payload storage never
/// exceeds record_cap. A fixed scratch block holds any overrun probe. Counts on
/// failure describe only the successfully decoded prefix; bytes_read includes
/// consumed invalid/probe bytes. This function has no wall-time or heap promise.
fn validate<R: Read>(reader: &mut R, limits: Limits) -> Result<Counts, Failure> {
    let mut counts = Counts::default();
    if !limits.is_within_profile() {
        return Err(failure(Code::Limits, counts));
    }
    let mut record = Vec::new();
    record
        .try_reserve_exact(limits.record_bytes)
        .map_err(|_| failure(Code::Allocation, counts))?;
    let mut scratch = [0_u8; SCRATCH_BYTES];

    loop {
        let aggregate_remaining = limits
            .aggregate_bytes
            .checked_sub(counts.bytes_read)
            .ok_or_else(|| failure(Code::Internal, counts))?;
        let aggregate_probe = aggregate_remaining
            .checked_add(1)
            .ok_or_else(|| failure(Code::CounterOverflow, counts))?;
        let record_probe = limits
            .record_bytes
            .checked_sub(record.len())
            .and_then(|remaining| remaining.checked_add(1))
            .ok_or_else(|| failure(Code::Internal, counts))?;
        let request_u64 = aggregate_probe.min(SCRATCH_BYTES as u64);
        let request = usize::try_from(request_u64)
            .map_err(|_| failure(Code::CounterOverflow, counts))?
            .min(record_probe);
        if request == 0 {
            return Err(failure(Code::Internal, counts));
        }

        let read = reader
            .read(&mut scratch[..request])
            .map_err(|_| failure(Code::StdinIo, counts))?;
        // Read implementors must honor the provided slice. Treat a broken
        // implementation as INTERNAL rather than indexing beyond scratch.
        if read > request {
            return Err(failure(Code::Internal, counts));
        }
        if read == 0 {
            if !record.is_empty() {
                return Err(failure(Code::FinalLfRequired, counts));
            }
            if counts.records == 0 {
                return Err(failure(Code::EmptyInput, counts));
            }
            return Ok(counts);
        }

        counts.bytes_read = counts
            .bytes_read
            .checked_add(read as u64)
            .ok_or_else(|| failure(Code::CounterOverflow, counts))?;
        if counts.bytes_read > limits.aggregate_bytes {
            return Err(failure(Code::AggregateLimit, counts));
        }

        for &byte in &scratch[..read] {
            if byte == b'\n' {
                if record.is_empty() {
                    return Err(failure(Code::EmptyRecord, counts));
                }
                if counts.records >= limits.records {
                    return Err(failure(Code::RecordCountLimit, counts));
                }
                let line = codex_rollout::parse_rollout_line_bytes(&record)
                    .map_err(|_| failure(Code::NativeDecode, counts))?;
                counts
                    .observe(&line.item, record.len())
                    .map_err(|code| failure(code, counts))?;
                drop(line);
                record.clear();
            } else {
                if counts.records >= limits.records {
                    return Err(failure(Code::RecordCountLimit, counts));
                }
                if record.len() >= limits.record_bytes {
                    return Err(failure(Code::RecordLimit, counts));
                }
                // Initial exact reservation covers this append. No growth
                // request is made for an excess record byte.
                record.push(byte);
            }
        }
    }
}

/// Render fixed keys, fixed codes and counters; never format an input/error.
fn render(code: Code, counts: &Counts) -> Result<Vec<u8>, Code> {
    let mut report = String::new();
    report
        .try_reserve_exact(MAX_REPORT_BYTES)
        .map_err(|_| Code::Allocation)?;
    write!(
        report,
        "{{\"code\":\"{}\",\"bytes_read\":{},\"records\":{},\
         \"max_record_bytes\":{},\"variants\":{{",
        code.label(),
        counts.bytes_read,
        counts.records,
        counts.max_record_bytes,
    )
    .map_err(|_| Code::Internal)?;
    for (index, name) in VARIANT_NAMES.iter().enumerate() {
        if index != 0 {
            report.push(',');
        }
        write!(report, "\"{}\":{}", name, counts.variants[index])
            .map_err(|_| Code::Internal)?;
    }
    write!(
        report,
        "}},\"compacted_with_replacement\":{},\
         \"replacement_items_total\":{},\"replacement_items_max\":{}}}\n",
        counts.compacted_with_replacement,
        counts.replacement_items_total,
        counts.replacement_items_max,
    )
    .map_err(|_| Code::Internal)?;
    if report.len() > MAX_REPORT_BYTES {
        return Err(Code::Internal);
    }
    Ok(report.into_bytes())
}

struct Prepared {
    output: Vec<u8>,
    exit: i32,
}

fn prepare_code(code: Code, counts: Counts) -> Prepared {
    match render(code, &counts) {
        Ok(output) => Prepared {
            output,
            exit: code.exit_code(),
        },
        Err(_) => Prepared {
            output: b"{\"code\":\"INTERNAL\"}\n".to_vec(),
            exit: Code::Internal.exit_code(),
        },
    }
}

fn prepare<R: Read>(reader: &mut R, limits: Limits) -> Prepared {
    match validate(reader, limits) {
        Ok(counts) => prepare_code(Code::Ok, counts),
        Err(error) => prepare_code(error.code, error.counts),
    }
}

/// A complete successful report is insufficient without a zero process exit.
fn emit<W: Write>(writer: &mut W, prepared: &Prepared) -> i32 {
    if writer.write_all(&prepared.output).is_err() || writer.flush().is_err() {
        return STDOUT_FAILURE_EXIT;
    }
    prepared.exit
}

fn main() {
    // Native serde/panic payloads may contain input. Suppress the hook and
    // catch ordinary unwinds before any output is sent. Abort/OOM is still a
    // non-success outcome; it is not a recoverable hard heap-bound mechanism.
    std::panic::set_hook(Box::new(|_| {}));
    let prepared = std::panic::catch_unwind(|| {
        if std::env::args_os().count() != 1 {
            return prepare_code(Code::Arguments, Counts::default());
        }
        let stdin = io::stdin();
        let mut reader = stdin.lock();
        prepare(&mut reader, Limits::PRODUCTION)
    })
    .unwrap_or_else(|_| prepare_code(Code::Internal, Counts::default()));

    let stdout = io::stdout();
    let mut writer = stdout.lock();
    let exit = emit(&mut writer, &prepared);
    drop(writer);
    std::process::exit(exit);
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    fn compact(payload: &str) -> Vec<u8> {
        format!(
            "{{\"timestamp\":\"2026-10-21T00:00:00Z\",\
             \"type\":\"compacted\",\"payload\":{payload}}}\n"
        )
        .into_bytes()
    }

    fn ordinary() -> Vec<u8> {
        compact(r#"{"message":"synthetic"}"#)
    }

    fn limits(total: usize, record: usize, records: u64) -> Limits {
        Limits {
            aggregate_bytes: total as u64,
            record_bytes: record,
            records,
        }
    }

    struct Chunked {
        bytes: Vec<u8>,
        position: usize,
        chunk: usize,
        fail_at: Option<usize>,
        max_requested: usize,
        calls: usize,
    }

    impl Chunked {
        fn new(bytes: Vec<u8>, chunk: usize) -> Self {
            Self {
                bytes,
                position: 0,
                chunk,
                fail_at: None,
                max_requested: 0,
                calls: 0,
            }
        }
    }

    impl Read for Chunked {
        fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
            self.calls += 1;
            self.max_requested = self.max_requested.max(buffer.len());
            if self.fail_at.is_some_and(|limit| self.position >= limit) {
                return Err(io::Error::other("SYNTHETIC_PRIVATE_IO_MARKER"));
            }
            let stop = self
                .bytes
                .len()
                .min(self.position.saturating_add(self.chunk))
                .min(self.position.saturating_add(buffer.len()))
                .min(self.fail_at.unwrap_or(usize::MAX));
            let length = stop.saturating_sub(self.position);
            buffer[..length]
                .copy_from_slice(&self.bytes[self.position..self.position + length]);
            self.position += length;
            Ok(length)
        }
    }

    #[test]
    fn one_record_exact_limits_succeeds_and_probes_eof() {
        let input = ordinary();
        let mut reader = Chunked::new(input.clone(), usize::MAX);
        let counts = validate(
            &mut reader,
            limits(input.len(), input.len() - 1, 1),
        )
        .unwrap();
        assert_eq!(counts.records, 1);
        assert_eq!(counts.variants[4], 1);
        assert_eq!(counts.bytes_read, input.len() as u64);
        assert_eq!(counts.max_record_bytes, (input.len() - 1) as u64);
        assert_eq!(reader.position, input.len());
        assert_eq!(reader.calls, 2);
        assert_eq!(counts.variants.iter().sum::<u64>(), counts.records);
    }

    #[test]
    fn zero_bytes_is_not_valid_history() {
        let error = validate(&mut &b""[..], Limits::PRODUCTION).unwrap_err();
        assert_eq!(error.code, Code::EmptyInput);
        assert_eq!(error.counts.bytes_read, 0);
    }

    #[test]
    fn empty_record_is_not_skipped() {
        let error = validate(&mut &b"\n"[..], Limits::PRODUCTION).unwrap_err();
        assert_eq!(error.code, Code::EmptyRecord);
    }

    #[test]
    fn whitespace_record_is_native_failure() {
        let error = validate(&mut &b" \t\n"[..], Limits::PRODUCTION).unwrap_err();
        assert_eq!(error.code, Code::NativeDecode);
    }

    #[test]
    fn terminal_lf_is_mandatory() {
        let mut input = ordinary();
        input.pop();
        let error = validate(&mut input.as_slice(), Limits::PRODUCTION)
            .unwrap_err();
        assert_eq!(error.code, Code::FinalLfRequired);
        assert_eq!(error.counts.records, 0);
    }

    #[test]
    fn valid_prefix_then_unterminated_record_is_not_success() {
        let mut input = ordinary();
        input.extend_from_slice(b"{");
        let error = validate(&mut input.as_slice(), Limits::PRODUCTION)
            .unwrap_err();
        assert_eq!(error.code, Code::FinalLfRequired);
        assert_eq!(error.counts.records, 1);
    }

    #[test]
    fn crlf_is_not_normalized_and_native_json_whitespace_is_accepted() {
        let mut input = ordinary();
        input.insert(input.len() - 1, b'\r');
        let counts = validate(&mut input.as_slice(), Limits::PRODUCTION).unwrap();
        assert_eq!(counts.records, 1);
        assert_eq!(counts.max_record_bytes, (input.len() - 1) as u64);
    }

    #[test]
    fn every_small_chunk_boundary_preserves_results() {
        let mut input = ordinary();
        input.extend(compact(r#"{"message":"snowman \u2603"}"#));
        let expected = validate(&mut input.as_slice(), Limits::PRODUCTION)
            .unwrap();
        for chunk in 1..=input.len() {
            let mut reader = Chunked::new(input.clone(), chunk);
            assert_eq!(
                validate(&mut reader, Limits::PRODUCTION).unwrap(),
                expected
            );
            assert!(reader.max_requested <= SCRATCH_BYTES);
        }
    }

    #[test]
    fn split_multibyte_utf8_is_validated_only_after_complete_framing() {
        let input = compact("{\"message\":\"☃\"}");
        let mut reader = Chunked::new(input, 1);
        assert_eq!(
            validate(&mut reader, Limits::PRODUCTION).unwrap().records,
            1
        );
    }

    #[test]
    fn aggregate_overrun_consumes_only_one_probe_byte() {
        let mut reader = Chunked::new(vec![b'x'; 100], usize::MAX);
        let error = validate(&mut reader, limits(12, 100, 10)).unwrap_err();
        assert_eq!(error.code, Code::AggregateLimit);
        assert_eq!(reader.position, 13);
        assert_eq!(reader.max_requested, 13);
        assert_eq!(error.counts.bytes_read, 13);
    }

    #[test]
    fn record_overrun_consumes_only_one_probe_byte_without_decode() {
        let mut reader = Chunked::new(vec![b'x'; 100], usize::MAX);
        let error = validate(&mut reader, limits(100, 7, 10)).unwrap_err();
        assert_eq!(error.code, Code::RecordLimit);
        assert_eq!(reader.position, 8);
        assert_eq!(reader.max_requested, 8);
        assert_eq!(error.counts.records, 0);
    }

    #[test]
    fn exact_record_limit_allows_its_lf() {
        let input = ordinary();
        let counts = validate(
            &mut input.as_slice(),
            limits(input.len(), input.len() - 1, 1),
        )
        .unwrap();
        assert_eq!(counts.records, 1);
    }

    #[test]
    fn one_more_aggregate_byte_is_refused() {
        let mut input = ordinary();
        let cap = input.len();
        input.push(b'x');
        let error = validate(
            &mut input.as_slice(),
            limits(cap, 1024, 10),
        )
        .unwrap_err();
        assert_eq!(error.code, Code::AggregateLimit);
        assert_eq!(error.counts.bytes_read, (cap + 1) as u64);
    }

    #[test]
    fn record_count_limit_refuses_next_record() {
        let mut input = ordinary();
        input.extend(ordinary());
        let error = validate(
            &mut input.as_slice(),
            limits(input.len(), 1024, 1),
        )
        .unwrap_err();
        assert_eq!(error.code, Code::RecordCountLimit);
        assert_eq!(error.counts.records, 1);
    }

    #[test]
    fn malformed_record_preserves_only_valid_prefix_and_no_marker() {
        let mut input = ordinary();
        input.extend_from_slice(
            b"{\"timestamp\":\"t\",\"type\":\"SYNTHETIC_PRIVATE_MARKER\"}\n",
        );
        let prepared = prepare(&mut input.as_slice(), Limits::PRODUCTION);
        let value: serde_json::Value = serde_json::from_slice(&prepared.output)
            .unwrap();
        assert_eq!(value["code"], "NATIVE_DECODE");
        assert_eq!(value["records"], 1);
        assert_ne!(prepared.exit, 0);
        assert!(
            !String::from_utf8(prepared.output)
                .unwrap()
                .contains("SYNTHETIC_PRIVATE_MARKER")
        );
    }

    #[test]
    fn invalid_utf8_is_native_failure() {
        let error = validate(&mut &b"{\xff}\n"[..], Limits::PRODUCTION)
            .unwrap_err();
        assert_eq!(error.code, Code::NativeDecode);
    }

    #[test]
    fn two_json_values_on_one_line_are_refused() {
        let error = validate(&mut &b"{} {}\n"[..], Limits::PRODUCTION)
            .unwrap_err();
        assert_eq!(error.code, Code::NativeDecode);
    }

    #[test]
    fn wrong_native_field_type_is_refused() {
        let input = compact(r#"{"message":7}"#);
        let error = validate(&mut input.as_slice(), Limits::PRODUCTION)
            .unwrap_err();
        assert_eq!(error.code, Code::NativeDecode);
    }

    #[test]
    fn compaction_none_and_empty_history_have_distinct_presence_counts() {
        let mut input = ordinary();
        input.extend(compact(
            r#"{"message":"synthetic","replacement_history":[]}"#,
        ));
        let counts = validate(&mut input.as_slice(), Limits::PRODUCTION)
            .unwrap();
        assert_eq!(counts.variants[4], 2);
        assert_eq!(counts.compacted_with_replacement, 1);
        assert_eq!(counts.replacement_items_total, 0);
        assert_eq!(counts.replacement_items_max, 0);
    }

    #[test]
    fn compaction_replacement_lengths_are_native_typed_counts() {
        let input = compact(
            r#"{"message":"synthetic","replacement_history":[
                {"type":"message","role":"user","content":[
                    {"type":"input_text","text":"synthetic"}
                ]},
                {"type":"message","role":"developer","content":[
                    {"type":"input_text","text":"synthetic"}
                ]}
            ]}"#,
        );
        // The input above is pretty JSON: remove formatting newlines so it
        // becomes one synthetic JSONL record without changing payload values.
        let input: Vec<u8> = input
            .iter()
            .enumerate()
            .filter(|(index, byte)| **byte != b'\n' || *index + 1 == input.len())
            .map(|(_, byte)| *byte)
            .collect();
        let counts = validate(&mut input.as_slice(), Limits::PRODUCTION)
            .unwrap();
        assert_eq!(counts.compacted_with_replacement, 1);
        assert_eq!(counts.replacement_items_total, 2);
        assert_eq!(counts.replacement_items_max, 2);
    }

    #[test]
    fn compaction_metadata_without_history_is_native_failure() {
        let input = compact(
            r#"{"message":"synthetic","replacement_history_metadata":[]}"#,
        );
        let error = validate(&mut input.as_slice(), Limits::PRODUCTION)
            .unwrap_err();
        assert_eq!(error.code, Code::NativeDecode);
    }

    #[test]
    fn compaction_metadata_length_mismatch_is_native_failure() {
        let input = compact(
            r#"{"message":"synthetic","replacement_history":[],
                "replacement_history_metadata":[{}]}"#,
        );
        let input: Vec<u8> = input
            .iter()
            .enumerate()
            .filter(|(index, byte)| **byte != b'\n' || *index + 1 == input.len())
            .map(|(_, byte)| *byte)
            .collect();
        let error = validate(&mut input.as_slice(), Limits::PRODUCTION)
            .unwrap_err();
        assert_eq!(error.code, Code::NativeDecode);
    }

    #[test]
    fn canonical_reordered_response_envelope_decodes() {
        let input = concat!(
            "{\"metadata\":{\"client_authored\":true},",
            "\"payload\":{\"type\":\"message\",\"role\":\"developer\",",
            "\"content\":[{\"type\":\"input_text\",\"text\":\"synthetic\"}]},",
            "\"type\":\"response_item\",\"ordinal\":9,",
            "\"timestamp\":\"2025-01-03T12:00:00.000Z\"}\n",
        );
        let counts = validate(&mut input.as_bytes(), Limits::PRODUCTION)
            .unwrap();
        assert_eq!(counts.variants[1], 1);
    }

    #[test]
    fn canonical_nested_decimal_event_decodes() {
        let input = concat!(
            "{\"timestamp\":\"2025-01-03T12:00:00.000Z\",",
            "\"ordinal\":7,\"type\":\"event_msg\",\"payload\":{",
            "\"type\":\"token_count\",\"info\":null,\"rate_limits\":{",
            "\"limit_id\":null,\"limit_name\":null,\"primary\":{",
            "\"used_percent\":0.0,\"window_minutes\":60,",
            "\"resets_at\":1800000000},\"secondary\":{",
            "\"used_percent\":12.5,\"window_minutes\":10080,",
            "\"resets_at\":1800100000},\"credits\":null,",
            "\"individual_limit\":null,\"spend_control_reached\":null,",
            "\"plan_type\":null,\"rate_limit_reached_type\":null}}}\n",
        );
        let counts = validate(&mut input.as_bytes(), Limits::PRODUCTION)
            .unwrap();
        assert_eq!(counts.variants[10], 1);
    }

    #[test]
    fn io_failure_preserves_prefix_and_hides_error_payload() {
        let input = ordinary();
        let length = input.len();
        let mut reader = Chunked::new(input, 1);
        reader.fail_at = Some(length);
        let prepared = prepare(&mut reader, Limits::PRODUCTION);
        let value: serde_json::Value = serde_json::from_slice(&prepared.output)
            .unwrap();
        assert_eq!(value["code"], "STDIN_IO");
        assert_eq!(value["records"], 1);
        assert_ne!(prepared.exit, 0);
        assert!(
            !String::from_utf8(prepared.output)
                .unwrap()
                .contains("SYNTHETIC_PRIVATE_IO_MARKER")
        );
    }

    #[test]
    fn counter_failure_does_not_partially_commit_a_record() {
        let input = ordinary();
        let line = codex_rollout::parse_rollout_line_bytes(
            &input[..input.len() - 1],
        )
        .unwrap();
        let mut counts = Counts {
            records: u64::MAX,
            ..Counts::default()
        };
        let before = counts;
        assert_eq!(
            counts.observe(&line.item, input.len() - 1),
            Err(Code::CounterOverflow)
        );
        assert_eq!(counts, before);
    }

    #[test]
    fn limits_above_profile_fail_before_any_read() {
        let mut reader = Chunked::new(ordinary(), 1);
        let invalid = Limits {
            aggregate_bytes: Limits::PRODUCTION.aggregate_bytes + 1,
            ..Limits::PRODUCTION
        };
        assert_eq!(
            validate(&mut reader, invalid).unwrap_err().code,
            Code::Limits
        );
        assert_eq!(reader.calls, 0);
    }

    #[test]
    fn fixed_report_schema_is_bounded_even_for_maximum_counters() {
        let counts = Counts {
            bytes_read: u64::MAX,
            records: u64::MAX,
            max_record_bytes: u64::MAX,
            variants: [u64::MAX; VARIANT_COUNT],
            compacted_with_replacement: u64::MAX,
            replacement_items_total: u64::MAX,
            replacement_items_max: u64::MAX,
        };
        let output = render(Code::Ok, &counts).unwrap();
        assert!(output.len() <= MAX_REPORT_BYTES);
        assert_eq!(output.last(), Some(&b'\n'));
        assert_eq!(output.iter().filter(|byte| **byte == b'\n').count(), 1);
        let value: serde_json::Value = serde_json::from_slice(&output).unwrap();
        let keys = value.as_object().unwrap();
        assert_eq!(keys.len(), 8);
        assert_eq!(value["variants"].as_object().unwrap().len(), 12);
    }

    struct BrokenWriter {
        remaining: usize,
        flush_failure: bool,
        captured: Vec<u8>,
    }

    impl Write for BrokenWriter {
        fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
            if self.remaining == 0 {
                return Err(io::Error::other("SYNTHETIC_WRITE_MARKER"));
            }
            let length = self.remaining.min(buffer.len());
            self.captured.extend_from_slice(&buffer[..length]);
            self.remaining -= length;
            Ok(length)
        }

        fn flush(&mut self) -> io::Result<()> {
            if self.flush_failure {
                return Err(io::Error::other("SYNTHETIC_FLUSH_MARKER"));
            }
            Ok(())
        }
    }

    #[test]
    fn output_write_failure_is_nonzero_even_after_partial_write() {
        let prepared = prepare_code(Code::Ok, Counts::default());
        let mut writer = BrokenWriter {
            remaining: 5,
            flush_failure: false,
            captured: Vec::new(),
        };
        assert_eq!(emit(&mut writer, &prepared), STDOUT_FAILURE_EXIT);
        assert_eq!(writer.captured.len(), 5);
    }

    #[test]
    fn output_flush_failure_is_nonzero_even_after_complete_report() {
        let prepared = prepare_code(Code::Ok, Counts::default());
        let mut writer = BrokenWriter {
            remaining: usize::MAX,
            flush_failure: true,
            captured: Vec::new(),
        };
        assert_eq!(emit(&mut writer, &prepared), STDOUT_FAILURE_EXIT);
        assert_eq!(writer.captured, prepared.output);
    }

    #[test]
    fn output_success_preserves_the_validation_exit_status() {
        let prepared = prepare_code(Code::NativeDecode, Counts::default());
        let mut output = Vec::new();
        assert_eq!(emit(&mut output, &prepared), 2);
        assert_eq!(output, prepared.output);
    }

    #[test]
    fn no_blank_record_is_accepted_after_valid_prefix() {
        let mut input = ordinary();
        input.push(b'\n');
        let error = validate(&mut input.as_slice(), Limits::PRODUCTION)
            .unwrap_err();
        assert_eq!(error.code, Code::EmptyRecord);
        assert_eq!(error.counts.records, 1);
    }
}
````

### 06_api_integration/261021_PY_NATIVE_LOCK_AUDIT_INT_v1.0_ANFL.py

````python
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
````

### 01_infrastructure/261021_YAML_NATIVE_VALIDATOR_CI_INT_v1.0_ANFL.yml

````yaml
# File: 261021_YAML_NATIVE_VALIDATOR_CI_INT_v1.0_ANFL.yml
# Product: The Reactor(TM)
# Status: implemented; remote compilation and synthetic tests pending.
# Public build inputs only. No recovery payloads, caches or Actions artifacts.
name: The Reactor native validator
on:
  workflow_dispatch:
permissions:
  contents: read
concurrency:
  group: anfl-native-validator
  cancel-in-progress: false
jobs:
  native:
    if: ${{ github.repository == 'MattSStack/anfl-public-validation' && github.event.repository.private == false }}
    runs-on: macos-15
    timeout-minutes: 90
    permissions:
      contents: write
    env:
      CODEX_REV: 83086188ae0614bd882818ff1aa23f3194962012
      HELPER_SHA: 65c4f7a10788052e94a5fa16a12d93d419fcbddfc27ecee6fa3577957f065ddf
      HELPER_REL: 06_api_integration/261021_RUST_NATIVE_ROLLOUT_VALIDATOR_INT_v1.0_ANFL.rs
      AUDITOR_SHA: c8a14e151547f4ce81aff8501f7e874ad3a5dc9b753a0af1691555b1648c5e1b
      AUDITOR_REL: 06_api_integration/261021_PY_NATIVE_LOCK_AUDIT_INT_v1.0_ANFL.py
      BINARY_NAME: 261021_BIN_NATIVE_ROLLOUT_VALIDATOR_ARM64_INT_v1.0_ANFL.bin
      MANIFEST_NAME: 261021_JSON_NATIVE_VALIDATOR_BUILD_INT_v1.0_ANFL.json
    steps:
      - name: Build and run native synthetic tests
        shell: zsh -e {0}
        env:
          REPO_IS_PRIVATE: ${{ github.event.repository.private }}
        run: |
          #!/bin/zsh
          set -euo pipefail
          [[ "$GITHUB_REPOSITORY" == "MattSStack/anfl-public-validation" ]]
          [[ "$REPO_IS_PRIVATE" == "false" ]]
          [[ "$(uname -m)" == "arm64" ]]
          [[ "$GITHUB_SHA" =~ '^[0-9a-f]{40};const startedAt=new Date().toISOString();let body=spec.body;if(spec.filePath){const f=fs.openSync(spec.filePath,fs.constants.O_RDONLY|fs.constants.O_NOFOLLOW);let b;try{if(fs.fstatSync(f).dev!==16777246)throw Error('externaldevice');b=fs.readFileSync(f);}finally{fs.closeSync(f);}if(crypto.createHash('sha256').update(b).digest('hex')!==spec.fileSha)throw Error('sourcepin');body={...body,content:b.toString('base64')};}const args=['api','--method',spec.method,spec.endpoint,'--input','-','--jq',spec.jq];const r=cp.spawnSync('/opt/homebrew/bin/gh',args,{input:JSON.stringify(body),encoding:'utf8',timeout:30000,maxBuffer:200000,env:{...process.env,GH_NO_UPDATE_NOTIFIER:'1',GH_NO_EXTENSION_UPDATE_NOTIFIER:'1',GH_PROMPT_DISABLED:'1'}});console.log(JSON.stringify({startedAt,completedAt:new Date().toISOString(),endpoint:spec.endpoint,exit:r.status,signal:r.signal,stdout:r.stdout,stderr:r.stderr,error:r.error?String(r.error):null})); ]]
          task_root="$(mktemp -d "$RUNNER_TEMP/anfl-native.XXXXXX")"
          publish_root="$RUNNER_TEMP/anfl-native-publish"
          mkdir "$publish_root"
          trap 'rm -rf -- "$task_root"' EXIT
          export CARGO_TARGET_DIR="$task_root/target"
          export CARGO_BUILD_JOBS=1
          export CARGO_INCREMENTAL=0
          export CARGO_PROFILE_DEV_DEBUG=0
          export CARGO_PROFILE_TEST_DEBUG=0
          export PUBLISH_ROOT="$publish_root"
          export TASK_ROOT="$task_root"
          curl --fail --silent --show-error --location \
            --proto '=https' --tlsv1.2 --max-time 60 --max-filesize 131072 \
            "https://raw.githubusercontent.com/$GITHUB_REPOSITORY/$GITHUB_SHA/$HELPER_REL" \
            --output "$task_root/validator.rs"
          actual_helper="$(shasum -a 256 "$task_root/validator.rs" | awk '{print $1}')"
          [[ "$actual_helper" == "$HELPER_SHA" ]]
          curl --fail --silent --show-error --location \
            --proto '=https' --tlsv1.2 --max-time 60 --max-filesize 131072 \
            "https://raw.githubusercontent.com/$GITHUB_REPOSITORY/$GITHUB_SHA/$AUDITOR_REL" \
            --output "$task_root/lock-audit.py"
          actual_auditor="$(shasum -a 256 "$task_root/lock-audit.py" | awk '{print $1}')"
          [[ "$actual_auditor" == "$AUDITOR_SHA" ]]
          PYTHONDONTWRITEBYTECODE=1 python3 "$task_root/lock-audit.py" --self-test \
            | tee "$publish_root/lock-audit-self-tests.json"
          git -c core.hooksPath=/dev/null init -q "$task_root/upstream"
          git -C "$task_root/upstream" remote add origin https://github.com/openai/codex.git
          git -C "$task_root/upstream" -c core.hooksPath=/dev/null \
            fetch --depth=1 --no-tags origin "$CODEX_REV"
          git -C "$task_root/upstream" -c core.hooksPath=/dev/null \
            checkout -q --detach FETCH_HEAD
          [[ "$(git -C "$task_root/upstream" rev-parse HEAD)" == "$CODEX_REV" ]]
          rustup toolchain install 1.95.0 --profile minimal
          rustc +1.95.0 -Vv > "$publish_root/rustc.txt"
          mkdir -p "$task_root/upstream/codex-rs/rollout/src/bin"
          cp "$task_root/validator.rs" \
            "$task_root/upstream/codex-rs/rollout/src/bin/anfl-rollout-stdin-validator.rs"
          original_lock="$(shasum -a 256 "$task_root/upstream/codex-rs/Cargo.lock" | awk '{print $1}')"
          [[ "$original_lock" == "17cdd862471774ba7aef836aaf0a0a3ed76bd23e503b1fc7278c89d1f2719d03" ]]
          cp "$task_root/upstream/codex-rs/Cargo.lock" "$publish_root/upstream-original.lock"
          cd "$task_root/upstream/codex-rs"
          cargo +1.95.0 tree -p codex-rollout --target aarch64-apple-darwin \
            --edges normal,build,dev --depth 1 > "$publish_root/dependency-tree.txt"
          if ! PYTHONDONTWRITEBYTECODE=1 python3 "$task_root/lock-audit.py" \
            "$publish_root/upstream-original.lock" \
            "$task_root/upstream/codex-rs/Cargo.lock" \
            "$task_root/upstream/codex-rs" > "$publish_root/lock-audit.json"; then
            cat "$publish_root/lock-audit.json"
            exit 1
          fi
          cat "$publish_root/lock-audit.json"
          python3 - <<'PY'
          import hashlib
          import json
          import os
          from pathlib import Path

          audit = json.loads(
              (Path(os.environ["PUBLISH_ROOT"]) / "lock-audit.json").read_text()
          )
          actual = (
              Path(os.environ["TASK_ROOT"]) / "upstream/codex-rs/Cargo.lock"
          ).read_bytes()
          assert audit["code"] == "OK"
          assert hashlib.sha256(actual).hexdigest() == audit["resolved_sha256"]
          PY
          shasum -a 256 "$task_root/upstream/codex-rs/Cargo.lock" \
            > "$publish_root/lock-before.txt"
          cargo +1.95.0 test --locked -p codex-rollout \
            --bin anfl-rollout-stdin-validator -- --test-threads=1 \
            2>&1 | tee "$publish_root/native-tests.txt"
          cargo +1.95.0 build --locked -p codex-rollout \
            --bin anfl-rollout-stdin-validator
          shasum -a 256 "$task_root/upstream/codex-rs/Cargo.lock" \
            > "$publish_root/lock-after.txt"
          cmp "$publish_root/lock-before.txt" "$publish_root/lock-after.txt"
          git -C "$task_root/upstream" diff --exit-code -- . ':!codex-rs/Cargo.lock'
          cp "$CARGO_TARGET_DIR/debug/anfl-rollout-stdin-validator" \
            "$publish_root/$BINARY_NAME"
          file "$publish_root/$BINARY_NAME" | tee "$publish_root/architecture.txt"
          otool -L "$publish_root/$BINARY_NAME" > "$publish_root/dependencies.txt"
          printf '%s\n' \
            '{"timestamp":"2026-10-21T00:00:00Z","type":"compacted","payload":{"message":"synthetic"}}' \
            | "$publish_root/$BINARY_NAME" > "$publish_root/sample.json"
          if printf '%s\n' '{"SYNTHETIC_PRIVATE_MARKER":true}' \
            | "$publish_root/$BINARY_NAME" > "$publish_root/invalid.json" \
              2> "$publish_root/invalid-stderr.txt"; then
            print -u2 'ERROR: malformed synthetic input unexpectedly succeeded'
            exit 1
          fi
          python3 - <<'PY'
          import hashlib
          import json
          import os
          from pathlib import Path
          import re
          import sys

          assert sys.version_info >= (3, 11)
          root = Path(os.environ["PUBLISH_ROOT"])
          binary = root / os.environ["BINARY_NAME"]
          size = binary.stat().st_size
          assert 0 < size <= 67_108_864
          sample = json.loads((root / "sample.json").read_text())
          invalid = json.loads((root / "invalid.json").read_text())
          assert sample["code"] == "OK" and sample["records"] == 1
          assert sample["variants"]["compacted"] == 1
          assert invalid["code"] == "NATIVE_DECODE"
          assert "SYNTHETIC_PRIVATE_MARKER" not in (
              (root / "invalid.json").read_text()
              + (root / "invalid-stderr.txt").read_text()
          )
          tests = (root / "native-tests.txt").read_text()
          assert re.search(r"test result: ok\. 32 passed; 0 failed;", tests)
          architecture = (root / "architecture.txt").read_text()
          assert "Mach-O 64-bit executable arm64" in architecture
          toolchain = (root / "rustc.txt").read_text()
          assert toolchain.startswith("rustc 1.95.0 ")
          assert "host: aarch64-apple-darwin" in toolchain
          sha = hashlib.sha256(binary.read_bytes()).hexdigest()
          lock_audit = json.loads((root / "lock-audit.json").read_text())
          assert lock_audit["code"] == "OK"
          assert lock_audit["resolved_sha256"] == (
              root / "lock-before.txt"
          ).read_text().split()[0]
          manifest = {
              "component": "The Reactor native typed record validator",
              "version": "1.0",
              "repository": os.environ["GITHUB_REPOSITORY"],
              "build_commit": os.environ["GITHUB_SHA"],
              "run_id": int(os.environ["GITHUB_RUN_ID"]),
              "run_attempt": int(os.environ["GITHUB_RUN_ATTEMPT"]),
              "upstream_commit": os.environ["CODEX_REV"],
              "helper_sha256": os.environ["HELPER_SHA"],
              "lock_sha256": (root / "lock-before.txt").read_text().split()[0],
              "lock_auditor_sha256": os.environ["AUDITOR_SHA"],
              "lock_audit": lock_audit,
              "lock_audit_self_tests": json.loads(
                  (root / "lock-audit-self-tests.json").read_text()
              ),
              "rustc": toolchain,
              "target": "aarch64-apple-darwin",
              "binary": {"name": binary.name, "bytes": size, "sha256": sha},
              "synthetic_tests_passed": 32,
              "cli_synthetic_success": sample,
              "cli_synthetic_failure_code": invalid["code"],
              "dynamic_dependencies": (root / "dependencies.txt").read_text(),
              "limits": {
                  "aggregate_bytes": 8_388_608,
                  "record_bytes": 4_194_304,
                  "records": 10_000,
                  "scratch_bytes": 8_192,
                  "report_bytes": 4_096,
              },
              "scope": "Public source and synthetic inputs only; typed compatibility only.",
              "protected_input_tested": False,
          }
          with (root / os.environ["MANIFEST_NAME"]).open("x") as stream:
              json.dump(manifest, stream, indent=2, sort_keys=True)
              stream.write("\n")
          assert (root / os.environ["MANIFEST_NAME"]).stat().st_size <= 65_536
          (root / "SHA256SUMS").write_text(
              sha + "  " + binary.name + "\n"
          )
          print(json.dumps({
              "native_tests_passed": 32,
              "binary_bytes": size,
              "binary_sha256": sha,
              "protected_input_tested": False,
          }))
          PY
          print 'SUCCESS: native compilation and 32 synthetic tests completed'
      - name: Stage bounded draft Release assets
        shell: zsh -e {0}
        env:
          GH_TOKEN: ${{ github.token }}
        run: |
          #!/bin/zsh
          set -euo pipefail
          [[ "$GITHUB_REPOSITORY" == "MattSStack/anfl-public-validation" ]]
          publish_root="$RUNNER_TEMP/anfl-native-publish"
          current_main="$(gh api "repos/$GITHUB_REPOSITORY/commits/main" --jq .sha)"
          [[ "$current_main" == "$GITHUB_SHA" ]]
          tag="anfl-native-261021-$GITHUB_RUN_ID-$GITHUB_RUN_ATTEMPT"
          notes="$publish_root/release-notes.txt"
          printf '%s\n' \
            'The Reactor native typed record validator, public synthetic qualification.' \
            "Upstream commit: $CODEX_REV" \
            "Build commit: $GITHUB_SHA" \
            "Helper SHA256: $HELPER_SHA" \
            'No protected input was accessed. Owner input admission remains separate.' \
            > "$notes"
          gh release create "$tag" --repo "$GITHUB_REPOSITORY" \
            --target "$GITHUB_SHA" --draft \
            --title 'The Reactor native validator 1.0' --notes-file "$notes" \
            "$publish_root/$BINARY_NAME" \
            "$publish_root/$MANIFEST_NAME" \
            "$publish_root/SHA256SUMS"
          print "SUCCESS: staged draft Release $tag"
````
