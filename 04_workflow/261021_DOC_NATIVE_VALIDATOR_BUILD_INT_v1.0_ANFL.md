---
file: "261021_DOC_NATIVE_VALIDATOR_BUILD_INT_v1.0_ANFL.md"
product: "The Reactor™"
created_utc: "2026-10-22T05:47:00Z"
version: "1.0"
layer: "04_workflow"
---

# BLUF

Build a bounded native typed-record validator from public inputs on a standard
public GitHub macOS runner. Recovery inputs are excluded from this build.
Compilation, tests and artifact qualification remain pending at document freeze.

# Executive

Canonical helper: 06_api_integration/261021_RUST_NATIVE_ROLLOUT_VALIDATOR_INT_v1.0_ANFL.rs.
Canonical workflow: 01_infrastructure/261021_YAML_NATIVE_VALIDATOR_CI_INT_v1.0_ANFL.yml.
The public build mirror installs that workflow under GitHub's required
.github/workflows/ directory. It does not move or retarget an existing recovery
component. The isolated build repository is MattSStack/anfl-public-validation.

The helper calls the official codex_rollout parser at commit
83086188ae0614bd882818ff1aa23f3194962012. CI adds only a new binary source inside
the existing codex-rollout package; it preserves the official Cargo.lock and
tracked source. Rust is pinned to 1.95.0. A successful native test and executable
receipt establish typed tooling availability, not recovery or input admission.

# Status

```yaml
component: "Public native validator build"
status: "implemented"
version: "1.0"
position_classification: "B"
layer: "06"
progress_percentage: 45
dependencies:
  - "Reviewed workflow publication and single dispatch"
  - "Actual native compilation and 32 synthetic test outcomes"
  - "Draft executable receipt and separate Owner qualification"
```

# Architecture

```text
Public helper + exact upstream commit + official lockfile
                       |
                       v
Standard public macos-15 ARM64 runner
1 Cargo job / 90 minute job limit / no cache or Actions artifacts
                       |
                       v
32 native unit tests + valid/invalid CLI synthetic checks
                       |
                       v
Draft Release: <=64 MiB binary + JSON provenance + SHA256
                       |
                       v
Owner verifies received bytes and separately qualifies actual input
```

# Implementation

Raw input limits are 8 MiB aggregate, 4 MiB per record, 10,000 records, and
8,192-byte scratch reads. Final LF and complete EOF are mandatory. The output
is at most 4,096 bytes of fixed keys/error codes and numeric counters. Any native
decode failure stops parsing. Counts represent the successfully decoded prefix;
only a full OK report plus exit zero establishes complete typed success.
Raw ingestion limits are not process heap or execution-time guarantees.

The source's 32 tests cover framing/chunk boundaries, caps/probes, typed native
compaction behavior, JSON compatibility, prefix/error privacy, counter updates,
and output write/flush failures. Static source review approved CI qualification;
it is not a substitute for running those tests.

# Ops / Runbook

Create only the isolated public build repository, publish the reviewed public
inputs, then dispatch once against its exact default-branch commit. Retain the
run ID before further action; an uncertain dispatch is not automatically retried.
Observe that run through the GitHub run/job API. The build records upstream,
source/lock/binary hashes, architecture, toolchain and synthetic outcomes.

The standard public macos-15 runner is documented free. Larger runners are paid.
The selected runner is ARM64 with 7 GB RAM and 14 GB SSD; resource fit is unproved.
[Runner contract](https://docs.github.com/en/actions/reference/runners/github-hosted-runners#standard-github-hosted-runners-for-public-repositories),
[Actions billing](https://docs.github.com/en/billing/concepts/product-billing/github-actions#free-use-of-github-actions).

This workflow uses no Actions artifact upload, cache, Packages or LFS. It stages
a small draft Release. Current Release docs state no aggregate size/bandwidth
limit; a 2024 GitHub Staff answer reports no financial impact for qualifying
Release files. Zero-cost Release delivery is an inference from this evidence,
rather than an express current pricing guarantee.
[Release quotas](https://docs.github.com/en/repositories/releasing-projects-on-github/about-releases#storage-and-bandwidth-quotas),
[Historical Staff answer](https://github.com/orgs/community/discussions/127096).

Threat notes: the job's contents:write permission grants general contents
authority in this isolated repository, not Release-only authority. GH_TOKEN is
exported only in the publishing step, after compilation and synthetic execution.
No OpenAI, Cloudflare, recovery, SSH or operator secret enters CI. The default
branch must still equal the reviewed build commit before draft publication;
no broader workflow permission is requested. Public code dependencies execute
on an ephemeral hosted runner. The native input consumer remains a separate
Owner-controlled resource and privacy boundary.

Draft assets are not public binary distribution. Public release publication and
any applicable distribution-notice review are separate from this build. No real
history, candidate, archive, T1 stream or credentials may be uploaded here.

Rollback: cancel the exact workflow run if required and leave its draft Release
unpublished. The helper and workflow are additive public build inputs; removing
them does not require a recovery cutover, runtime restart or data movement.

# Complete review snapshot

The following full contents are review snapshots. Execution uses the canonical
files identified above. Their exact pins are recorded with the build receipt.

## Native helper — complete source

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
codex-rollout manifest has no autobins=false; no dependency/lockfile edit
is required. Run only separately authorized public-source CI builds.

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

## Workflow — complete source

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
      HELPER_SHA: a2e592334910bcfb83cd01d8830e603281ce5bfbf5970f2c37ada97bca736cd3
      HELPER_REL: 06_api_integration/261021_RUST_NATIVE_ROLLOUT_VALIDATOR_INT_v1.0_ANFL.rs
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
          [[ "$GITHUB_SHA" =~ '^[0-9a-f]{40}
````
 ]]
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
          shasum -a 256 "$task_root/upstream/codex-rs/Cargo.lock" \
            > "$publish_root/lock-before.txt"
          cd "$task_root/upstream/codex-rs"
          cargo +1.95.0 test --locked -p codex-rollout \
            --bin anfl-rollout-stdin-validator -- --test-threads=1 \
            2>&1 | tee "$publish_root/native-tests.txt"
          cargo +1.95.0 build --locked -p codex-rollout \
            --bin anfl-rollout-stdin-validator
          shasum -a 256 "$task_root/upstream/codex-rs/Cargo.lock" \
            > "$publish_root/lock-after.txt"
          cmp "$publish_root/lock-before.txt" "$publish_root/lock-after.txt"
          git -C "$task_root/upstream" diff --exit-code
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
