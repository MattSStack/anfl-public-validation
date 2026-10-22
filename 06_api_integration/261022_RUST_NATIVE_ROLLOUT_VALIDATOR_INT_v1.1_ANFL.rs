/*
File: 261022_RUST_NATIVE_ROLLOUT_VALIDATOR_INT_v1.1_ANFL.rs
Product: The Reactor(TM)
Component: Native typed rollout stdin validator
Version: 1.1
Layer: 06_api_integration
Implementation status: source implemented; compilation and tests NOT yet run.
Upstream: openai/codex@83086188ae0614bd882818ff1aa23f3194962012
Toolchain: official codex-rs/rust-toolchain.toml pins Rust 1.95.0.
Integration: copy this PUBLIC source into the pinned workspace at
codex-rs/rollout/src/bin/anfl-rollout-stdin-validator.rs. The published
codex-rollout manifest autodiscovers the binary without new dependencies.
CI separately audits the upstream release-version lock stamps before building.
Run only separately authorized public-source CI builds.

BLUF: Default mode decodes each bounded record; explicit native scan mode
also tests the storage-reader boundary using ModelContextScan itself.
The scan mode is REQUESTED behavior, not attestation of Paginated mode,
input order, source-tail coverage, lineage, or reconstructed window state.
Newest-to-oldest input and complete source-tail coverage are caller
assertions. Native Complete grants only the supplied-input boundary.
Native scanner records are retained within the existing aggregate and
record caps; these are not hard heap limits. No actual input is admitted.
No files, configuration, authentication, app-server, or rollout loader.
Typed compatibility is not lossless fidelity, lineage, compaction eligibility,
consumer independence, or recovery proof. Native decoding can allocate more
than the raw record; an external supervisor owns process memory/time limits.
Do not pass private recovery data to public CI.

Architecture: stdin -> bounded raw framing -> native typed record -> counters
-> drop record -> one bounded JSON report. Any invalid record stops the stream.
Runtime accepts no arguments for decode-only, or exactly the literal
--model-context-scan for native storage-boundary mode. No path args.
Scan success requires push Complete and full-input LF/EOF validation.
Older records remain strictly decoded and counted but are never fed to
the completed scanner. Only EOF allows finish chronological counters.
*/
#![forbid(unsafe_code)]

use std::fmt::Write as _;
use std::io::{self, Read, Write};

use codex_rollout::{
    CompactedItem, ModelContextScan, ModelContextScanProgress, RolloutItem,
};

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
    BoundaryOk,
    Arguments,
    Limits,
    EmptyInput,
    EmptyRecord,
    BoundaryNotFound,
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
            Self::BoundaryOk => "MODEL_CONTEXT_BOUNDARY_OK",
            Self::Arguments => "ARGUMENTS",
            Self::Limits => "LIMITS",
            Self::EmptyInput => "EMPTY_INPUT",
            Self::EmptyRecord => "EMPTY_RECORD",
            Self::BoundaryNotFound => "NATIVE_BOUNDARY_NOT_FOUND",
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
            Self::Ok | Self::BoundaryOk => 0,
            Self::StdinIo => 3,
            Self::Allocation | Self::CounterOverflow | Self::Internal => 4,
            _ => 2,
        }
    }
}

/// Requested execution mode; it does not authenticate history mode or coverage.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Mode {
    Decode,
    ModelContextScan,
}

fn selected_mode<I>(mut arguments: I) -> Result<Mode, Code>
where
    I: Iterator<Item = std::ffi::OsString>,
{
    match arguments.next() {
        None => Ok(Mode::Decode),
        Some(flag)
            if flag == std::ffi::OsStr::new("--model-context-scan")
                && arguments.next().is_none() =>
        {
            Ok(Mode::ModelContextScan)
        }
        _ => Err(Code::Arguments),
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
    older_records_ignored_by_scanner: u64,
}

#[derive(Clone, Copy)]
struct ScanMetadata {
    chronological_records: u64,
    chronological_first_is_compacted: bool,
}

struct Validated {
    counts: Counts,
    scan_metadata: Option<ScanMetadata>,
    older_records_ignored_by_scanner: u64,
}

fn failure(code: Code, counts: Counts) -> Failure {
    Failure {
        code,
        counts,
        older_records_ignored_by_scanner: 0,
    }
}

/// Validate stdin-like bytes without opening paths or retaining whole history.
///
/// Reads consume at most aggregate_cap + 1 bytes. Record payload storage never
/// exceeds record_cap. A fixed scratch block holds any overrun probe. Counts on
/// failure describe only the successfully decoded prefix; bytes_read includes
/// consumed invalid/probe bytes. This function has no wall-time or heap promise.
fn validate<R: Read>(reader: &mut R, limits: Limits) -> Result<Counts, Failure> {
    validate_mode(reader, limits, Mode::Decode).map(|validated| validated.counts)
}

/// Feed native records in caller-supplied order, never sorting or inferring it.
///
/// Scan mode consumes the supplied records newest-to-oldest as asserted by its
/// caller. After native Complete, older records are still decoded strictly
/// but never pushed to the scanner. finish follows successful whole-input
/// LF/EOF validation and is never itself a boundary grant.
fn validate_mode<R: Read>(
    reader: &mut R,
    limits: Limits,
    mode: Mode,
) -> Result<Validated, Failure> {
    let mut older_records_ignored_by_scanner = 0;
    let mut result = validate_mode_inner(
        reader,
        limits,
        mode,
        &mut older_records_ignored_by_scanner,
    );
    if let Err(error) = &mut result {
        error.older_records_ignored_by_scanner = older_records_ignored_by_scanner;
    }
    result
}

fn validate_mode_inner<R: Read>(
    reader: &mut R,
    limits: Limits,
    mode: Mode,
    older_records_ignored_by_scanner: &mut u64,
) -> Result<Validated, Failure> {
    let mut counts = Counts::default();
    let mut scanner = (mode == Mode::ModelContextScan)
        .then(ModelContextScan::default);
    let mut boundary_complete = false;
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
            let scan_metadata = if let Some(scanner) = scanner {
                if !boundary_complete {
                    return Err(failure(Code::BoundaryNotFound, counts));
                }
                let chronological = scanner.finish();
                let length = u64::try_from(chronological.len())
                    .map_err(|_| failure(Code::CounterOverflow, counts))?;
                let first_compacted = matches!(
                    chronological.first(),
                    Some(RolloutItem::Compacted(_))
                );
                let selected_records = counts.records
                    .checked_sub(*older_records_ignored_by_scanner)
                    .ok_or_else(|| failure(Code::Internal, counts))?;
                if length != selected_records || !first_compacted {
                    return Err(failure(Code::Internal, counts));
                }
                drop(chronological);
                Some(ScanMetadata {
                    chronological_records: length,
                    chronological_first_is_compacted: first_compacted,
                })
            } else {
                None
            };
            return Ok(Validated {
                counts,
                scan_metadata,
                older_records_ignored_by_scanner:
                    *older_records_ignored_by_scanner,
            });
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
                if boundary_complete {
                    *older_records_ignored_by_scanner =
                        (*older_records_ignored_by_scanner)
                            .checked_add(1)
                            .ok_or_else(|| {
                                failure(Code::CounterOverflow, counts)
                            })?;
                    drop(line);
                } else if let Some(scanner) = scanner.as_mut() {
                    boundary_complete = matches!(
                        scanner.push(line.item),
                        ModelContextScanProgress::Complete
                    );
                } else {
                    drop(line);
                }
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

/// Add only fixed mode labels and chronological counters, never input fields.
fn render_scan(
    code: Code,
    counts: &Counts,
    metadata: Option<ScanMetadata>,
    older_records_ignored_by_scanner: u64,
) -> Result<Vec<u8>, Code> {
    let mut output = render(code, counts)?;
    if !output.ends_with(b"}\n") {
        return Err(Code::Internal);
    }
    output.truncate(output.len() - 2);
    let complete = u8::from(metadata.is_some());
    let chronological_records = metadata
        .map(|value| value.chronological_records)
        .unwrap_or(0);
    let first_compacted = u8::from(
        metadata.is_some_and(|value| value.chronological_first_is_compacted)
    );
    let suffix = format!(
        ",\"mode\":\"model_context_scan_newest_first\",\
         \"native_boundary_complete\":{},\"chronological_records\":{},\
         \"chronological_first_is_compacted\":{},\
         \"older_records_ignored_by_scanner\":{}}}\n",
        complete,
        chronological_records,
        first_compacted,
        older_records_ignored_by_scanner,
    );
    if output.len().saturating_add(suffix.len()) > MAX_REPORT_BYTES {
        return Err(Code::Internal);
    }
    output.extend_from_slice(suffix.as_bytes());
    Ok(output)
}

fn prepare_mode<R: Read>(
    reader: &mut R,
    limits: Limits,
    mode: Mode,
) -> Prepared {
    if mode == Mode::Decode {
        return prepare(reader, limits);
    }
    let (code, counts, metadata, older) =
        match validate_mode(reader, limits, mode) {
            Ok(validated) => (
                Code::BoundaryOk,
                validated.counts,
                validated.scan_metadata,
                validated.older_records_ignored_by_scanner,
            ),
            Err(error) => (
                error.code,
                error.counts,
                None,
                error.older_records_ignored_by_scanner,
            ),
        };
    match render_scan(code, &counts, metadata, older) {
        Ok(output) => Prepared {
            output,
            exit: code.exit_code(),
        },
        Err(_) => prepare_code(Code::Internal, Counts::default()),
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
        let mode = match selected_mode(std::env::args_os().skip(1)) {
            Ok(mode) => mode,
            Err(code) => return prepare_code(code, Counts::default()),
        };
        let stdin = io::stdin();
        let mut reader = stdin.lock();
        prepare_mode(&mut reader, Limits::PRODUCTION, mode)
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

    // v1.1 focused tests follow the 32 byte-preserved v1.0 test bodies.
    fn complete_boundary() -> Vec<u8> {
        compact(
            r#"{"message":"synthetic","replacement_history":[],
                "window_number":0}"#,
        )
        .into_iter()
        .filter(|byte| *byte != b'\n')
        .chain(std::iter::once(b'\n'))
        .collect()
    }

    fn ordinary_tail() -> Vec<u8> {
        concat!(
            "{\"timestamp\":\"2026-10-21T00:00:00Z\",\"type\":\"event_msg\",",
            "\"payload\":{\"type\":\"warning\",\"message\":\"synthetic\"}}\n",
        )
        .as_bytes()
        .to_vec()
    }

    fn scan_report(mut input: &[u8]) -> (Prepared, serde_json::Value) {
        let prepared = prepare_mode(
            &mut input,
            Limits::PRODUCTION,
            Mode::ModelContextScan,
        );
        let value = serde_json::from_slice(&prepared.output).unwrap();
        (prepared, value)
    }

    #[test]
    fn v11_default_mode_is_decode_only() {
        assert_eq!(
            selected_mode(std::iter::empty::<std::ffi::OsString>()),
            Ok(Mode::Decode)
        );
    }

    #[test]
    fn v11_scan_mode_requires_exact_literal_flag() {
        assert_eq!(
            selected_mode(
                [std::ffi::OsString::from("--model-context-scan")].into_iter()
            ),
            Ok(Mode::ModelContextScan)
        );
    }

    #[test]
    fn v11_paths_extra_flags_and_extra_args_are_refused() {
        for values in [
            vec!["synthetic.jsonl"],
            vec!["--other"],
            vec!["--model-context-scan", "synthetic.jsonl"],
            vec!["--model-context-scan", "--model-context-scan"],
        ] {
            assert_eq!(
                selected_mode(values.into_iter().map(std::ffi::OsString::from)),
                Err(Code::Arguments)
            );
        }
    }

    #[test]
    fn v11_default_report_remains_decoder_only() {
        let input = ordinary();
        let prepared = prepare_mode(
            &mut input.as_slice(),
            Limits::PRODUCTION,
            Mode::Decode,
        );
        let expected = prepare(&mut input.as_slice(), Limits::PRODUCTION);
        assert_eq!(prepared.exit, 0);
        assert_eq!(prepared.output, expected.output);
        let value: serde_json::Value =
            serde_json::from_slice(&prepared.output).unwrap();
        assert_eq!(value["code"], "OK");
        assert_eq!(value.as_object().unwrap().len(), 8);
        assert!(value.get("native_boundary_complete").is_none());
    }

    #[test]
    fn v11_native_complete_then_eof_has_chronological_metadata() {
        let mut input = ordinary_tail();
        input.extend(ordinary_tail());
        input.extend(complete_boundary());
        let (prepared, value) = scan_report(&input);
        assert_eq!(prepared.exit, 0);
        assert_eq!(value["code"], "MODEL_CONTEXT_BOUNDARY_OK");
        assert_eq!(value["mode"], "model_context_scan_newest_first");
        assert_eq!(value["records"], 3);
        assert_eq!(value["variants"]["event_msg"], 2);
        assert_eq!(value["variants"]["compacted"], 1);
        assert_eq!(value["native_boundary_complete"], 1);
        assert_eq!(value["chronological_records"], 3);
        assert_eq!(value["chronological_first_is_compacted"], 1);
        assert_eq!(value["compacted_with_replacement"], 1);
        assert_eq!(value["replacement_items_total"], 0);
        assert!(prepared.output.len() <= MAX_REPORT_BYTES);
    }

    #[test]
    fn v11_finish_without_native_complete_never_grants_boundary() {
        let (prepared, value) = scan_report(&ordinary_tail());
        assert_ne!(prepared.exit, 0);
        assert_eq!(value["code"], "NATIVE_BOUNDARY_NOT_FOUND");
        assert_eq!(value["native_boundary_complete"], 0);
        assert_eq!(value["chronological_records"], 0);
    }

    #[test]
    fn v11_newer_incomplete_compaction_suppresses_older_complete() {
        let mut input = ordinary();
        input.extend(complete_boundary());
        let (prepared, value) = scan_report(&input);
        assert_ne!(prepared.exit, 0);
        assert_eq!(value["code"], "NATIVE_BOUNDARY_NOT_FOUND");
        assert_eq!(value["records"], 2);
        assert_eq!(value["native_boundary_complete"], 0);
    }

    #[test]
    fn v11_missing_replacement_cannot_complete() {
        let input = compact(r#"{"message":"synthetic","window_number":0}"#);
        let (prepared, value) = scan_report(&input);
        assert_ne!(prepared.exit, 0);
        assert_eq!(value["code"], "NATIVE_BOUNDARY_NOT_FOUND");
    }

    #[test]
    fn v11_missing_window_cannot_complete() {
        let input = compact(
            r#"{"message":"synthetic","replacement_history":[]}"#,
        );
        let (prepared, value) = scan_report(&input);
        assert_ne!(prepared.exit, 0);
        assert_eq!(value["code"], "NATIVE_BOUNDARY_NOT_FOUND");
    }

    #[test]
    fn v11_older_record_is_decoded_but_not_fed_after_complete() {
        let mut input = complete_boundary();
        input.extend(ordinary());
        let (prepared, value) = scan_report(&input);
        assert_eq!(prepared.exit, 0);
        assert_eq!(value["code"], "MODEL_CONTEXT_BOUNDARY_OK");
        assert_eq!(value["records"], 2);
        assert_eq!(value["variants"]["compacted"], 2);
        assert_eq!(value["native_boundary_complete"], 1);
        assert_eq!(value["chronological_records"], 1);
        assert_eq!(value["older_records_ignored_by_scanner"], 1);
    }

    #[test]
    fn v11_malformed_older_record_after_complete_still_refuses_input() {
        let mut input = complete_boundary();
        input.extend_from_slice(b"SYNTHETIC_OLDER_PRIVATE_MARKER\n");
        let (prepared, value) = scan_report(&input);
        assert_ne!(prepared.exit, 0);
        assert_eq!(value["code"], "NATIVE_DECODE");
        assert_eq!(value["records"], 1);
        assert_eq!(value["older_records_ignored_by_scanner"], 0);
        assert!(
            !String::from_utf8(prepared.output)
                .unwrap()
                .contains("SYNTHETIC_OLDER_PRIVATE_MARKER")
        );
    }

    #[test]
    fn v11_extra_lf_after_complete_is_refused() {
        let mut input = complete_boundary();
        input.push(b'\n');
        let (prepared, value) = scan_report(&input);
        assert_ne!(prepared.exit, 0);
        assert_eq!(value["code"], "EMPTY_RECORD");
    }

    #[test]
    fn v11_native_complete_without_successful_eof_is_not_success() {
        let input = complete_boundary();
        let length = input.len();
        let mut reader = Chunked::new(input, 1);
        reader.fail_at = Some(length);
        let prepared = prepare_mode(
            &mut reader,
            Limits::PRODUCTION,
            Mode::ModelContextScan,
        );
        let value: serde_json::Value =
            serde_json::from_slice(&prepared.output).unwrap();
        assert_ne!(prepared.exit, 0);
        assert_eq!(value["code"], "STDIN_IO");
        assert_eq!(value["native_boundary_complete"], 0);
        assert_eq!(value["chronological_records"], 0);
    }

    #[test]
    fn v11_boundary_requires_final_lf() {
        let mut input = complete_boundary();
        input.pop();
        let (prepared, value) = scan_report(&input);
        assert_ne!(prepared.exit, 0);
        assert_eq!(value["code"], "FINAL_LF_REQUIRED");
        assert_eq!(value["native_boundary_complete"], 0);
    }

    #[test]
    fn v11_exact_caps_allow_native_complete_and_eof() {
        let input = complete_boundary();
        let prepared = prepare_mode(
            &mut input.as_slice(),
            limits(input.len(), input.len() - 1, 1),
            Mode::ModelContextScan,
        );
        let value: serde_json::Value =
            serde_json::from_slice(&prepared.output).unwrap();
        assert_eq!(prepared.exit, 0);
        assert_eq!(value["code"], "MODEL_CONTEXT_BOUNDARY_OK");
    }

    #[test]
    fn v11_aggregate_cap_can_refuse_before_complete() {
        let input = complete_boundary();
        let mut reader = Chunked::new(input.clone(), usize::MAX);
        let prepared = prepare_mode(
            &mut reader,
            limits(input.len() - 1, input.len(), 1),
            Mode::ModelContextScan,
        );
        let value: serde_json::Value =
            serde_json::from_slice(&prepared.output).unwrap();
        assert_ne!(prepared.exit, 0);
        assert_eq!(value["code"], "AGGREGATE_LIMIT");
        assert_eq!(reader.position, input.len());
        assert_eq!(value["native_boundary_complete"], 0);
    }

    #[test]
    fn v11_record_cap_refuses_boundary_without_grant() {
        let input = complete_boundary();
        let prepared = prepare_mode(
            &mut input.as_slice(),
            limits(input.len(), input.len() - 2, 1),
            Mode::ModelContextScan,
        );
        let value: serde_json::Value =
            serde_json::from_slice(&prepared.output).unwrap();
        assert_ne!(prepared.exit, 0);
        assert_eq!(value["code"], "RECORD_LIMIT");
        assert_eq!(value["native_boundary_complete"], 0);
    }

    #[test]
    fn v11_record_count_cap_refuses_before_boundary() {
        let mut input = ordinary_tail();
        input.extend(complete_boundary());
        let prepared = prepare_mode(
            &mut input.as_slice(),
            limits(input.len(), 1024, 1),
            Mode::ModelContextScan,
        );
        let value: serde_json::Value =
            serde_json::from_slice(&prepared.output).unwrap();
        assert_ne!(prepared.exit, 0);
        assert_eq!(value["code"], "RECORD_COUNT_LIMIT");
        assert_eq!(value["native_boundary_complete"], 0);
    }

    #[test]
    fn v11_boundary_result_is_independent_of_read_chunking() {
        let mut input = ordinary_tail();
        input.extend(complete_boundary());
        let expected = prepare_mode(
            &mut input.as_slice(),
            Limits::PRODUCTION,
            Mode::ModelContextScan,
        );
        assert_eq!(expected.exit, 0);
        for chunk in 1..=input.len() {
            let mut reader = Chunked::new(input.clone(), chunk);
            let prepared = prepare_mode(
                &mut reader,
                Limits::PRODUCTION,
                Mode::ModelContextScan,
            );
            assert_eq!(prepared.exit, expected.exit);
            assert_eq!(prepared.output, expected.output);
            assert!(reader.max_requested <= SCRATCH_BYTES);
        }
    }

    #[test]
    fn v11_native_decode_error_payload_is_not_reported_in_scan_mode() {
        let input = b"{\"timestamp\":\"t\",\"type\":\"SYNTHETIC_SECRET\"}\n";
        let (prepared, value) = scan_report(input);
        assert_ne!(prepared.exit, 0);
        assert_eq!(value["code"], "NATIVE_DECODE");
        assert_eq!(value["native_boundary_complete"], 0);
        assert!(
            !String::from_utf8(prepared.output)
                .unwrap()
                .contains("SYNTHETIC_SECRET")
        );
    }

    #[test]
    fn v11_older_header_preserves_same_selected_native_context() {
        let mut selected_input = ordinary_tail();
        selected_input.extend(complete_boundary());
        let (_, selected) = scan_report(&selected_input);
        let header = concat!(
            "{\"timestamp\":\"2026-10-21T00:00:00Z\",",
            "\"type\":\"session_meta\",\"payload\":{",
            "\"id\":\"00000000-0000-7000-8000-000000000001\",",
            "\"timestamp\":\"2026-10-21T00:00:00Z\",",
            "\"cwd\":\"/synthetic\",\"originator\":\"synthetic\",",
            "\"cli_version\":\"0.0.0\",\"history_mode\":\"paginated\"}}\n",
        );
        selected_input.extend_from_slice(header.as_bytes());
        let (prepared, with_header) = scan_report(&selected_input);
        assert_eq!(prepared.exit, 0);
        assert_eq!(with_header["code"], "MODEL_CONTEXT_BOUNDARY_OK");
        assert_eq!(with_header["native_boundary_complete"], 1);
        assert_eq!(
            with_header["chronological_records"],
            selected["chronological_records"]
        );
        assert_eq!(
            with_header["chronological_first_is_compacted"],
            selected["chronological_first_is_compacted"]
        );
        assert_eq!(with_header["records"], 3);
        assert_eq!(with_header["variants"]["session_meta"], 1);
        assert_eq!(with_header["older_records_ignored_by_scanner"], 1);
    }

    #[test]
    fn v11_older_valid_prefix_count_survives_later_io_failure() {
        let mut input = complete_boundary();
        input.extend(ordinary_tail());
        let length = input.len();
        let mut reader = Chunked::new(input, 1);
        reader.fail_at = Some(length);
        let prepared = prepare_mode(
            &mut reader,
            Limits::PRODUCTION,
            Mode::ModelContextScan,
        );
        let value: serde_json::Value =
            serde_json::from_slice(&prepared.output).unwrap();
        assert_ne!(prepared.exit, 0);
        assert_eq!(value["code"], "STDIN_IO");
        assert_eq!(value["records"], 2);
        assert_eq!(value["older_records_ignored_by_scanner"], 1);
        assert_eq!(value["native_boundary_complete"], 0);
        assert_eq!(value["chronological_records"], 0);
    }
}
