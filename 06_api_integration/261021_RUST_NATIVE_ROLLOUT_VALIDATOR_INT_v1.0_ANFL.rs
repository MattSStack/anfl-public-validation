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
