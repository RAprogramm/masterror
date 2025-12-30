// SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>
//
// SPDX-License-Identifier: MIT

#[cfg(any(feature = "backtrace", feature = "tracing"))]
use std::sync::Mutex;
use std::{
    borrow::Cow,
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
    io::{Error as IoError, ErrorKind as IoErrorKind},
    sync::Arc
};

#[cfg(feature = "std")]
use anyhow::Error as AnyhowError;

#[cfg(feature = "std")]
#[derive(Debug)]
struct AnyhowSource(AnyhowError);

#[cfg(feature = "std")]
impl Display for AnyhowSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

#[cfg(feature = "std")]
impl StdError for AnyhowSource {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.0.source()
    }
}

#[cfg(feature = "backtrace")]
use super::core::{reset_backtrace_preference, set_backtrace_preference_override};

#[cfg(feature = "backtrace")]
static BACKTRACE_ENV_GUARD: Mutex<()> = Mutex::new(());

#[cfg(feature = "tracing")]
static TELEMETRY_GUARD: Mutex<()> = Mutex::new(());

#[cfg(feature = "tracing")]
mod telemetry_support {
    use std::{
        fmt,
        sync::{Arc, Mutex}
    };

    use tracing::{
        Dispatch, Event, Subscriber,
        field::{Field, Visit}
    };
    use tracing_subscriber::{
        Registry,
        layer::{Context, Layer, SubscriberExt}
    };

    #[derive(Default, Clone)]
    pub(super) struct RecordedEvent {
        pub(super) trace_id: Option<String>,
        pub(super) code:     Option<String>,
        pub(super) category: Option<String>
    }

    pub(super) type RecordedEvents = Arc<Mutex<Vec<RecordedEvent>>>;

    pub(super) fn new_recording_dispatch() -> (Dispatch, RecordedEvents) {
        let events = Arc::new(Mutex::new(Vec::new()));
        let layer = RecordingLayer {
            events: events.clone()
        };
        let subscriber = Registry::default().with(layer);
        let dispatch = Dispatch::new(subscriber);
        (dispatch, events)
    }

    struct RecordingLayer {
        events: RecordedEvents
    }

    impl<S> Layer<S> for RecordingLayer
    where
        S: Subscriber
    {
        fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
            if event.metadata().target() != "masterror::error" {
                return;
            }
            let mut record = RecordedEvent::default();
            event.record(&mut EventVisitor {
                record: &mut record
            });
            self.events.lock().expect("events lock").push(record);
        }
    }

    struct EventVisitor<'a> {
        record: &'a mut RecordedEvent
    }

    impl<'a> Visit for EventVisitor<'a> {
        fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
            let normalized = normalize_debug(value);
            match field.name() {
                "trace_id" => self.record.trace_id = Some(normalized),
                "code" => self.record.code = Some(normalized),
                "category" => self.record.category = Some(normalized),
                _ => {}
            }
        }
    }

    fn normalize_debug(value: &dyn fmt::Debug) -> String {
        let mut rendered = format!("{value:?}");
        while let Some(stripped) = rendered
            .strip_prefix("Some(")
            .and_then(|s| s.strip_suffix(')'))
        {
            rendered = stripped.to_owned();
        }
        rendered.trim_matches('"').to_owned()
    }
}

use super::{AppError, FieldRedaction, FieldValue, MessageEditPolicy, field};
use crate::{AppCode, AppErrorKind, Context, ErrorResponse, ResultExt};

// --- Helpers -------------------------------------------------------------

/// Assert helper: kind matches and message is Some(s).
fn assert_err_with_msg(err: AppError, expected: AppErrorKind, msg: &str) {
    assert!(
        matches!(err.kind, k if k == expected),
        "expected kind {:?}, got {:?}",
        expected,
        err.kind
    );
    assert_eq!(err.message.as_deref(), Some(msg));
}

/// Assert helper: kind matches and message is None.
fn assert_err_bare(err: AppError, expected: AppErrorKind) {
    assert!(
        matches!(err.kind, k if k == expected),
        "expected kind {:?}, got {:?}",
        expected,
        err.kind
    );
    assert!(err.message.is_none());
}

#[test]
fn constructors_match_kinds() {
    assert_err_with_msg(
        AppError::not_found("missing"),
        AppErrorKind::NotFound,
        "missing"
    );
    assert_err_with_msg(
        AppError::validation("invalid"),
        AppErrorKind::Validation,
        "invalid"
    );
    assert_err_with_msg(
        AppError::unauthorized("need token"),
        AppErrorKind::Unauthorized,
        "need token"
    );
    assert_err_with_msg(
        AppError::forbidden("no access"),
        AppErrorKind::Forbidden,
        "no access"
    );
    assert_err_with_msg(AppError::conflict("dup"), AppErrorKind::Conflict, "dup");
    assert_err_with_msg(
        AppError::bad_request("bad"),
        AppErrorKind::BadRequest,
        "bad"
    );
    assert_err_with_msg(
        AppError::rate_limited("slow"),
        AppErrorKind::RateLimited,
        "slow"
    );
    assert_err_with_msg(
        AppError::telegram_auth("fail"),
        AppErrorKind::TelegramAuth,
        "fail"
    );
    assert_err_with_msg(AppError::internal("oops"), AppErrorKind::Internal, "oops");
    assert_err_with_msg(AppError::service("down"), AppErrorKind::Service, "down");
    assert_err_with_msg(AppError::config("bad cfg"), AppErrorKind::Config, "bad cfg");
    assert_err_with_msg(
        AppError::turnkey("turnkey"),
        AppErrorKind::Turnkey,
        "turnkey"
    );
    assert_err_with_msg(
        AppError::timeout("timeout"),
        AppErrorKind::Timeout,
        "timeout"
    );
    assert_err_with_msg(AppError::network("net"), AppErrorKind::Network, "net");
    assert_err_with_msg(
        AppError::dependency_unavailable("dep"),
        AppErrorKind::DependencyUnavailable,
        "dep"
    );
    assert_err_with_msg(
        AppError::service_unavailable("dep"),
        AppErrorKind::DependencyUnavailable,
        "dep"
    );
    assert_err_with_msg(
        AppError::serialization("ser"),
        AppErrorKind::Serialization,
        "ser"
    );
    assert_err_with_msg(
        AppError::deserialization("deser"),
        AppErrorKind::Deserialization,
        "deser"
    );
    assert_err_with_msg(
        AppError::external_api("external"),
        AppErrorKind::ExternalApi,
        "external"
    );
    assert_err_with_msg(AppError::queue("queue"), AppErrorKind::Queue, "queue");
    assert_err_with_msg(AppError::cache("cache"), AppErrorKind::Cache, "cache");
}

#[cfg(feature = "std")]
#[test]
fn with_context_attaches_plain_source() {
    let err = AppError::internal("boom").with_context(IoError::from(IoErrorKind::Other));
    let source = err.source_ref().expect("stored source");
    assert!(source.is::<IoError>());
    assert_eq!(source.to_string(), IoErrorKind::Other.to_string());
}

#[cfg(feature = "std")]
#[test]
fn with_context_accepts_anyhow_error() {
    let upstream: AnyhowError = anyhow::anyhow!("context failed");
    let err = AppError::service("downstream").with_context(AnyhowSource(upstream));
    let source = err.source_ref().expect("stored source");
    let stored = source
        .downcast_ref::<AnyhowSource>()
        .expect("anyhow source");
    assert_eq!(stored.0.to_string(), "context failed");
}

#[test]
fn database_accepts_optional_message() {
    let with_msg = AppError::database_with_message("db down");
    assert_err_with_msg(with_msg, AppErrorKind::Database, "db down");
    let via_option = AppError::database(Some(Cow::Borrowed("db down")));
    assert_err_with_msg(via_option, AppErrorKind::Database, "db down");
    let without = AppError::database(None);
    assert_err_bare(without, AppErrorKind::Database);
}

#[test]
fn bare_sets_kind_without_message() {
    assert_err_bare(
        AppError::bare(AppErrorKind::Internal),
        AppErrorKind::Internal
    );
}

#[test]
fn render_message_returns_borrowed_label_for_bare_errors() {
    let err = AppError::bare(AppErrorKind::Forbidden);
    let rendered = err.render_message();
    assert!(matches!(
        rendered,
        Cow::Borrowed(label) if label == AppErrorKind::Forbidden.label()
    ));
}

#[test]
fn retry_and_www_authenticate_are_attached() {
    let err = AppError::internal("boom")
        .with_retry_after_secs(30)
        .with_www_authenticate("Bearer");
    assert_eq!(err.retry.unwrap().after_seconds, 30);
    assert_eq!(err.www_authenticate.as_deref(), Some("Bearer"));
}

#[test]
fn context_moves_dynamic_code_without_cloning() {
    let dynamic_code =
        AppCode::try_new(String::from("THIRD_PARTY_FAILURE")).expect("valid dynamic code");
    let expected_ptr = dynamic_code.as_str().as_ptr();
    let err = Result::<(), IoError>::Err(IoError::from(IoErrorKind::Other))
        .ctx(|| Context::new(AppErrorKind::Service).code(dynamic_code))
        .unwrap_err();
    assert_eq!(err.code.as_str().as_ptr(), expected_ptr);
}

#[test]
fn render_message_does_not_allocate_for_borrowed_str() {
    let err = AppError::new(AppErrorKind::BadRequest, "borrowed");
    let rendered = err.render_message();
    assert!(matches!(rendered, Cow::Borrowed("borrowed")));
    assert!(std::ptr::eq(rendered.as_ref(), "borrowed"));
}

#[test]
fn metadata_and_code_are_preserved() {
    let err = AppError::service("downstream")
        .with_field(field::str("request_id", "abc-123"))
        .with_field(field::i64("attempt", 2))
        .with_code(AppCode::Service);
    assert_eq!(err.code, AppCode::Service);
    let metadata = err.metadata();
    assert_eq!(metadata.len(), 2);
    assert_eq!(
        metadata.get("request_id"),
        Some(&FieldValue::Str(Cow::Borrowed("abc-123")))
    );
    assert_eq!(metadata.get("attempt"), Some(&FieldValue::I64(2)));
}

#[test]
fn custom_literal_codes_flow_into_responses() {
    let custom = AppCode::new("INVALID_JSON");
    let err = AppError::bad_request("invalid").with_code(custom.clone());
    assert_eq!(err.code, custom);
    let response: ErrorResponse = err.into();
    assert_eq!(response.code, custom);
}

#[test]
fn dynamic_codes_flow_into_responses() {
    let custom = AppCode::try_new(String::from("THIRD_PARTY_FAILURE")).expect("valid code");
    let err = AppError::service("down").with_code(custom.clone());
    assert_eq!(err.code, custom);
    let response: ErrorResponse = err.into();
    assert_eq!(response.code, custom);
}

#[cfg(feature = "serde_json")]
#[test]
fn with_details_json_attaches_payload() {
    use serde_json::json;
    let payload = json!({"field": "email"});
    let err = AppError::validation("invalid").with_details_json(payload.clone());
    assert_eq!(err.details, Some(payload));
}

#[cfg(feature = "serde_json")]
#[test]
fn with_details_serialization_failure_is_bad_request() {
    use serde::{Serialize, Serializer};
    struct Failing;
    impl Serialize for Failing {
        fn serialize<S>(&self, _: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer
        {
            Err(serde::ser::Error::custom("nope"))
        }
    }
    let err = AppError::internal("boom")
        .with_details(Failing)
        .expect_err("should fail");
    assert!(matches!(err.kind, AppErrorKind::BadRequest));
}

#[cfg(not(feature = "serde_json"))]
#[test]
fn with_details_text_attaches_payload() {
    let err = AppError::internal("boom").with_details_text("retry later");
    assert_eq!(err.details.as_deref(), Some("retry later"));
}

#[test]
fn context_with_preserves_default_redaction() {
    let err = super::Context::new(AppErrorKind::Service)
        .with(field::str("request_id", "abc-123"))
        .into_error(DummyError);
    let metadata = err.metadata();
    assert_eq!(metadata.len(), 1);
    assert_eq!(
        metadata.get("request_id"),
        Some(&FieldValue::Str(Cow::Borrowed("abc-123")))
    );
    assert_eq!(metadata.redaction("request_id"), Some(FieldRedaction::None));
}

#[test]
fn context_redact_field_overrides_policy() {
    let err = super::Context::new(AppErrorKind::Service)
        .with(field::str("token", "super-secret"))
        .redact_field("token", FieldRedaction::Redact)
        .into_error(DummyError);
    let metadata = err.metadata();
    assert_eq!(
        metadata.get("token"),
        Some(&FieldValue::Str(Cow::Borrowed("super-secret")))
    );
    assert_eq!(metadata.redaction("token"), Some(FieldRedaction::Redact));
}

#[test]
fn context_redact_field_before_insertion_applies_policy() {
    let err = super::Context::new(AppErrorKind::Service)
        .redact_field("token", FieldRedaction::Hash)
        .with(field::str("token", "super-secret"))
        .into_error(DummyError);
    let metadata = err.metadata();
    assert_eq!(
        metadata.get("token"),
        Some(&FieldValue::Str(Cow::Borrowed("super-secret")))
    );
    assert_eq!(metadata.redaction("token"), Some(FieldRedaction::Hash));
}

#[test]
fn context_redact_field_mut_applies_policies() {
    let mut context = super::Context::new(AppErrorKind::Service);
    let _ = context.redact_field_mut("token", FieldRedaction::Hash);
    context = context.with(field::str("token", "super-secret"));
    let err = context.into_error(DummyError);
    let metadata = err.metadata();
    assert_eq!(
        metadata.get("token"),
        Some(&FieldValue::Str(Cow::Borrowed("super-secret")))
    );
    assert_eq!(metadata.redaction("token"), Some(FieldRedaction::Hash));
}

#[test]
fn context_with_uses_latest_matching_policy() {
    let err = super::Context::new(AppErrorKind::Service)
        .redact_field("token", FieldRedaction::Hash)
        .redact_field("token", FieldRedaction::Redact)
        .with(field::str("token", "super-secret"))
        .into_error(DummyError);
    let metadata = err.metadata();
    assert_eq!(
        metadata.get("token"),
        Some(&FieldValue::Str(Cow::Borrowed("super-secret")))
    );
    assert_eq!(metadata.redaction("token"), Some(FieldRedaction::Redact));
}

#[test]
fn app_error_redact_field_updates_metadata() {
    let err = AppError::internal("boom")
        .with_field(field::str("api_key", "key"))
        .redact_field("api_key", FieldRedaction::Hash);
    assert_eq!(
        err.metadata().redaction("api_key"),
        Some(FieldRedaction::Hash)
    );
    assert_eq!(
        err.metadata().get("api_key"),
        Some(&FieldValue::Str(Cow::Borrowed("key")))
    );
}

#[derive(Debug)]
struct DummyError;

impl Display for DummyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("dummy")
    }
}

impl StdError for DummyError {}

#[test]
fn source_is_preserved_without_extra_allocation() {
    let source = Arc::new(DummyError);
    let err = AppError::internal("boom").with_source_arc(source.clone());
    assert_eq!(Arc::strong_count(&source), 2);
    let stored = err.source_ref().expect("source");
    let stored_dummy = stored
        .downcast_ref::<DummyError>()
        .expect("dummy should be preserved");
    assert!(std::ptr::eq(stored_dummy, &*source));
}

#[test]
fn error_chain_is_preserved() {
    #[derive(Debug)]
    struct NestedError {
        inner: DummyError
    }
    impl Display for NestedError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.inner.fmt(f)
        }
    }
    impl StdError for NestedError {
        fn source(&self) -> Option<&(dyn StdError + 'static)> {
            Some(&self.inner)
        }
    }
    let err = AppError::internal("boom").with_source(NestedError {
        inner: DummyError
    });
    let top_source = StdError::source(&err).expect("top source");
    assert!(top_source.is::<NestedError>());
    let nested = top_source.source().expect("nested source");
    assert!(nested.is::<DummyError>());
}

#[cfg(feature = "backtrace")]
fn with_backtrace_preference<F: FnOnce()>(value: Option<bool>, test: F) {
    let _guard = BACKTRACE_ENV_GUARD.lock().expect("env guard");
    reset_backtrace_preference();
    set_backtrace_preference_override(value);
    test();
    set_backtrace_preference_override(None);
    reset_backtrace_preference();
}

#[cfg(feature = "backtrace")]
#[test]
fn backtrace_respects_disabled_env() {
    with_backtrace_preference(Some(false), || {
        let err = AppError::internal("boom");
        assert!(err.backtrace().is_none());
    });
}

#[cfg(feature = "backtrace")]
#[test]
fn backtrace_enabled_when_env_requests() {
    with_backtrace_preference(Some(true), || {
        let err = AppError::internal("boom");
        assert!(err.backtrace().is_some());
    });
}

#[test]
fn redactable_policy_is_exposed() {
    let err = AppError::internal("boom").redactable();
    assert!(matches!(err.edit_policy, MessageEditPolicy::Redact));
}

/// Smoke test to ensure `log()` is callable; tracing output isn't asserted.
#[test]
fn log_uses_kind_and_code() {
    let err = AppError::internal("boom");
    err.log();
}

#[cfg(feature = "tracing")]
#[test]
fn telemetry_emits_single_tracing_event_with_trace_id() {
    let _guard = TELEMETRY_GUARD.lock().expect("telemetry guard");
    use telemetry_support::new_recording_dispatch;
    use tracing::{callsite::rebuild_interest_cache, dispatcher};
    let (dispatch, events) = new_recording_dispatch();
    let events = events.clone();
    dispatcher::with_default(&dispatch, || {
        rebuild_interest_cache();
        log_mdc::insert("trace_id", "trace-123");
        let err = AppError::internal("boom");
        err.log();
        log_mdc::remove("trace_id");
        let events = events.lock().expect("events lock");
        assert_eq!(events.len(), 1, "expected exactly one tracing event");
        let event = &events[0];
        assert_eq!(event.code.as_deref(), Some(AppCode::Internal.as_str()));
        assert_eq!(event.category.as_deref(), Some("Internal"));
        assert!(
            event
                .trace_id
                .as_deref()
                .is_some_and(|value| value.contains("trace-123"))
        );
    });
}

#[cfg(feature = "tracing")]
#[test]
fn telemetry_flushes_after_subscriber_install() {
    let _guard = TELEMETRY_GUARD.lock().expect("telemetry guard");
    use telemetry_support::new_recording_dispatch;
    use tracing::{callsite::rebuild_interest_cache, dispatcher};
    let (dispatch, events) = new_recording_dispatch();
    let events_clone = events.clone();
    dispatcher::with_default(&dispatch, || {
        rebuild_interest_cache();
        let err = AppError::internal("boom");
        err.log();
        drop(err);
        let events = events_clone.lock().expect("events lock");
        assert_eq!(
            events.len(),
            1,
            "expected telemetry after subscriber install"
        );
        let event = &events[0];
        assert_eq!(event.code.as_deref(), Some(AppCode::Internal.as_str()));
        assert_eq!(event.category.as_deref(), Some("Internal"));
    });
}

#[cfg(feature = "metrics")]
#[test]
fn metrics_counter_is_incremented_once() {
    use std::{
        collections::HashMap,
        sync::{Arc, Mutex}
    };

    use metrics::{
        Counter, CounterFn, Gauge, Histogram, Key, KeyName, Metadata, Recorder, SharedString, Unit
    };
    #[derive(Clone, Debug, Eq, PartialEq, Hash)]
    struct CounterKey {
        name:   String,
        labels: Vec<(String, String)>
    }
    impl CounterKey {
        fn new(name: String, labels: Vec<(String, String)>) -> Self {
            Self {
                name,
                labels
            }
        }
    }
    type CounterMap = HashMap<CounterKey, u64>;
    type SharedCounterMap = Arc<Mutex<CounterMap>>;
    #[derive(Clone)]
    struct MetricsCounterHandle {
        key:    CounterKey,
        counts: SharedCounterMap
    }
    impl CounterFn for MetricsCounterHandle {
        fn increment(&self, value: u64) {
            let mut map = self.counts.lock().expect("counter map");
            *map.entry(self.key.clone()).or_default() += value;
        }
        fn absolute(&self, value: u64) {
            let mut map = self.counts.lock().expect("counter map");
            map.insert(self.key.clone(), value);
        }
    }
    struct CountingRecorder {
        counts: SharedCounterMap
    }
    impl Recorder for CountingRecorder {
        fn describe_counter(
            &self,
            _key: KeyName,
            _unit: Option<Unit>,
            _description: SharedString
        ) {
        }
        fn describe_gauge(&self, _key: KeyName, _unit: Option<Unit>, _description: SharedString) {}
        fn describe_histogram(
            &self,
            _key: KeyName,
            _unit: Option<Unit>,
            _description: SharedString
        ) {
        }
        fn register_counter(&self, key: &Key, _metadata: &Metadata<'_>) -> Counter {
            let labels = key
                .labels()
                .map(|label| (label.key().to_owned(), label.value().to_owned()))
                .collect::<Vec<_>>();
            let counter_key = CounterKey::new(key.name().to_owned(), labels);
            Counter::from_arc(Arc::new(MetricsCounterHandle {
                key:    counter_key,
                counts: self.counts.clone()
            }))
        }
        fn register_gauge(&self, _key: &Key, _metadata: &Metadata<'_>) -> Gauge {
            Gauge::noop()
        }
        fn register_histogram(&self, _key: &Key, _metadata: &Metadata<'_>) -> Histogram {
            Histogram::noop()
        }
    }
    use std::sync::OnceLock;
    static RECORDER_COUNTS: OnceLock<SharedCounterMap> = OnceLock::new();
    let counts = RECORDER_COUNTS
        .get_or_init(|| {
            let counts = Arc::new(Mutex::new(HashMap::new()));
            metrics::set_global_recorder(CountingRecorder {
                counts: counts.clone()
            })
            .expect("install recorder");
            counts
        })
        .clone();
    counts.lock().expect("counter map").clear();
    let err = AppError::forbidden("denied");
    err.log();
    let key = CounterKey::new(
        "error_total".to_owned(),
        vec![
            ("code".to_owned(), AppCode::Forbidden.as_str().to_owned()),
            ("category".to_owned(), "Forbidden".to_owned()),
        ]
    );
    let counts = counts.lock().expect("counter map");
    assert_eq!(counts.get(&key).copied(), Some(1));
}

#[test]
fn result_alias_is_generic() {
    let default_result: super::AppResult<u8> = Ok(1);
    let custom_result: super::AppResult<u8, &'static str> = Ok(2);
    assert!(matches!(default_result, Ok(value) if value == 1));
    assert!(matches!(custom_result, Ok(value) if value == 2));
}

#[test]
fn app_error_fits_result_budget() {
    let size = size_of::<AppError>();
    assert!(
        size <= 128,
        "AppError grew to {size} bytes; keep the Err variant lean"
    );
}

#[test]
#[cfg(feature = "std")]
fn error_chain_iterates_through_sources() {
    let io_err = IoError::other("disk offline");
    let app_err = AppError::internal("db down").with_context(io_err);
    let chain: Vec<_> = app_err.chain().collect();
    assert_eq!(chain.len(), 2);
    let first_err = chain[0].to_string();
    assert!(
        first_err.contains("Internal")
            || first_err.contains("INTERNAL")
            || first_err.contains("Error:")
    );
    #[cfg(feature = "colored")]
    assert!(first_err.contains("db down"));
    #[cfg(not(feature = "colored"))]
    assert!(first_err.contains("Internal"));
    assert_eq!(chain[1].to_string(), "disk offline");
}

#[test]
#[cfg(feature = "std")]
fn error_chain_single_error() {
    let err = AppError::bad_request("missing field");
    let chain: Vec<_> = err.chain().collect();
    assert_eq!(chain.len(), 1);
    let err_str = chain[0].to_string();
    assert!(err_str.contains("Bad") || err_str.contains("BAD"));
    #[cfg(feature = "colored")]
    {
        assert!(chain[0].to_string().contains("Bad request"));
        assert!(chain[0].to_string().contains("missing field"));
    }
}

#[test]
#[cfg(feature = "std")]
fn error_chain_multiple_sources() {
    let root = IoError::new(IoErrorKind::NotFound, "file not found");
    let wrapped = IoError::other(format!("config error: {}", root));
    let app_err = AppError::internal("startup failed").with_context(wrapped);
    let chain: Vec<_> = app_err.chain().collect();
    assert_eq!(chain.len(), 2);
}

#[test]
#[cfg(feature = "std")]
fn root_cause_returns_deepest_error() {
    let io_err = IoError::other("disk offline");
    let app_err = AppError::internal("db down").with_context(io_err);
    let root = app_err.root_cause();
    assert_eq!(root.to_string(), "disk offline");
}

#[test]
#[cfg(feature = "std")]
fn root_cause_returns_self_when_no_source() {
    let err = AppError::timeout("operation timed out");
    let root = err.root_cause();
    let root_str = root.to_string();
    assert!(
        root_str.contains("timed out")
            || root_str.contains("TIMEOUT")
            || root_str.contains("Timeout")
    );
    #[cfg(feature = "colored")]
    assert!(root_str.contains("operation"));
    #[cfg(not(feature = "colored"))]
    assert!(root_str.contains("Timeout") || root_str.contains("timed out"));
}

#[test]
#[cfg(feature = "std")]
fn is_checks_source_type() {
    let io_err = IoError::other("disk offline");
    let app_err = AppError::internal("db down").with_context(io_err);
    assert!(app_err.is::<IoError>());
    let anyhow_err = anyhow::anyhow!("test error");
    let anyhow_app_err = AppError::internal("wrapped").with_context(AnyhowSource(anyhow_err));
    assert!(!anyhow_app_err.is::<IoError>());
}

#[test]
#[cfg(feature = "std")]
fn is_returns_false_when_no_source() {
    let err = AppError::not_found("user not found");
    assert!(!err.is::<IoError>());
    assert!(!err.is::<AnyhowSource>());
}

#[test]
#[cfg(feature = "std")]
fn downcast_ref_retrieves_source() {
    let io_err = IoError::other("disk offline");
    let app_err = AppError::internal("db down").with_context(io_err);
    let retrieved = app_err.downcast_ref::<IoError>().expect("should downcast");
    assert_eq!(retrieved.to_string(), "disk offline");
}

#[test]
#[cfg(feature = "std")]
fn downcast_ref_returns_none_when_wrong_type() {
    let io_err = IoError::other("disk offline");
    let app_err = AppError::internal("db down").with_context(io_err);
    assert!(app_err.downcast_ref::<AnyhowSource>().is_none());
}

#[test]
#[cfg(feature = "std")]
fn downcast_ref_returns_none_when_no_source() {
    let err = AppError::not_found("user not found");
    assert!(err.downcast_ref::<IoError>().is_none());
}

#[test]
#[cfg(feature = "colored")]
fn colored_display_bare_error_without_message() {
    let err = AppError::bare(AppErrorKind::Internal);
    let output = format!("{}", err);
    assert!(output.contains("Internal server error"));
    assert!(output.contains("Code:"));
    assert!(output.contains("INTERNAL"));
}

#[test]
#[cfg(feature = "colored")]
fn colored_display_deep_error_chain() {
    use crate::field;
    #[derive(Debug)]
    struct CustomError {
        msg:    String,
        source: Option<IoError>
    }
    impl Display for CustomError {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            write!(f, "{}", self.msg)
        }
    }
    impl StdError for CustomError {
        fn source(&self) -> Option<&(dyn StdError + 'static)> {
            self.source.as_ref().map(|e| e as &(dyn StdError + 'static))
        }
    }
    let root = IoError::other("disk full");
    let mid = CustomError {
        msg:    "write failed".to_string(),
        source: Some(root)
    };
    let top = AppError::internal("operation failed")
        .with_context(mid)
        .with_field(field::str("operation", "backup"));
    let output = format!("{}", top);
    assert!(output.contains("Internal server error"));
    assert!(output.contains("INTERNAL"));
    assert!(output.contains("operation failed"));
    assert!(output.contains("Caused by"));
    assert!(output.contains("write failed"));
    assert!(output.contains("disk full"));
    assert!(output.contains("Context:"));
    assert!(output.contains("operation"));
    assert!(output.contains("backup"));
}

#[test]
#[cfg(feature = "std")]
fn with_metadata_replaces_all_metadata() {
    use crate::{Metadata, field};
    let err = AppError::internal("test")
        .with_field(field::str("key1", "value1"))
        .with_field(field::u64("key2", 42));
    let new_metadata = Metadata::from_fields(vec![
        field::str("new_key", "new_value"),
        field::u64("count", 100),
    ]);
    let err = err.with_metadata(new_metadata);
    let metadata = err.metadata();
    assert!(metadata.get("new_key").is_some());
    assert!(metadata.get("count").is_some());
    assert!(metadata.get("key1").is_none());
    assert!(metadata.get("key2").is_none());
}

#[test]
#[cfg(feature = "std")]
fn with_context_handles_arc_source() {
    let io_err = IoError::other("network down");
    let arc_source: Arc<dyn StdError + Send + Sync + 'static> = Arc::new(io_err);
    let err1 = AppError::internal("first")
        .with_context(Arc::clone(&arc_source) as Arc<dyn StdError + Send + Sync>);
    let err2 = AppError::internal("second").with_context(arc_source);
    assert!(err1.source_ref().is_some());
    assert!(err2.source_ref().is_some());
    assert_eq!(err1.source_ref().unwrap().to_string(), "network down");
    assert_eq!(err2.source_ref().unwrap().to_string(), "network down");
}

#[test]
#[cfg(feature = "std")]
fn with_context_handles_boxed_arc_downcast() {
    let io_err = IoError::other("boxed arc");
    let arc_source: Arc<dyn StdError + Send + Sync + 'static> = Arc::new(io_err);
    let boxed_arc: Box<Arc<dyn StdError + Send + Sync + 'static>> = Box::new(arc_source);
    let err = AppError::internal("test").with_context(*boxed_arc);
    assert!(err.source_ref().is_some());
    assert_eq!(err.source_ref().unwrap().to_string(), "boxed arc");
}

#[test]
fn not_found_constructor_creates_correct_kind() {
    let err = AppError::not_found("resource not found");
    assert_eq!(err.kind, AppErrorKind::NotFound);
    assert_eq!(err.message.as_deref(), Some("resource not found"));
}

#[test]
fn not_found_accepts_string() {
    let err = AppError::not_found("test".to_string());
    assert_eq!(err.message.as_deref(), Some("test"));
}

#[test]
fn not_found_accepts_cow_borrowed() {
    let err = AppError::not_found(Cow::Borrowed("borrowed"));
    assert_eq!(err.message.as_deref(), Some("borrowed"));
}

#[test]
fn not_found_accepts_cow_owned() {
    let err = AppError::not_found(Cow::Owned("owned".to_string()));
    assert_eq!(err.message.as_deref(), Some("owned"));
}

#[test]
fn validation_constructor_creates_correct_kind() {
    let err = AppError::validation("invalid input");
    assert_eq!(err.kind, AppErrorKind::Validation);
    assert_eq!(err.message.as_deref(), Some("invalid input"));
}

#[test]
fn unauthorized_constructor_creates_correct_kind() {
    let err = AppError::unauthorized("missing token");
    assert_eq!(err.kind, AppErrorKind::Unauthorized);
    assert_eq!(err.message.as_deref(), Some("missing token"));
}

#[test]
fn forbidden_constructor_creates_correct_kind() {
    let err = AppError::forbidden("access denied");
    assert_eq!(err.kind, AppErrorKind::Forbidden);
    assert_eq!(err.message.as_deref(), Some("access denied"));
}

#[test]
fn conflict_constructor_creates_correct_kind() {
    let err = AppError::conflict("resource exists");
    assert_eq!(err.kind, AppErrorKind::Conflict);
    assert_eq!(err.message.as_deref(), Some("resource exists"));
}

#[test]
fn bad_request_constructor_creates_correct_kind() {
    let err = AppError::bad_request("malformed request");
    assert_eq!(err.kind, AppErrorKind::BadRequest);
    assert_eq!(err.message.as_deref(), Some("malformed request"));
}

#[test]
fn rate_limited_constructor_creates_correct_kind() {
    let err = AppError::rate_limited("too many requests");
    assert_eq!(err.kind, AppErrorKind::RateLimited);
    assert_eq!(err.message.as_deref(), Some("too many requests"));
}

#[test]
fn telegram_auth_constructor_creates_correct_kind() {
    let err = AppError::telegram_auth("invalid telegram auth");
    assert_eq!(err.kind, AppErrorKind::TelegramAuth);
    assert_eq!(err.message.as_deref(), Some("invalid telegram auth"));
}

#[test]
fn internal_constructor_creates_correct_kind() {
    let err = AppError::internal("unexpected error");
    assert_eq!(err.kind, AppErrorKind::Internal);
    assert_eq!(err.message.as_deref(), Some("unexpected error"));
}

#[test]
fn service_constructor_creates_correct_kind() {
    let err = AppError::service("service failure");
    assert_eq!(err.kind, AppErrorKind::Service);
    assert_eq!(err.message.as_deref(), Some("service failure"));
}

#[test]
fn database_constructor_with_none_creates_correct_kind() {
    let err = AppError::database(None);
    assert_eq!(err.kind, AppErrorKind::Database);
    assert!(err.message.is_none());
}

#[test]
fn database_constructor_with_some_creates_correct_kind() {
    let err = AppError::database(Some(Cow::Borrowed("connection failed")));
    assert_eq!(err.kind, AppErrorKind::Database);
    assert_eq!(err.message.as_deref(), Some("connection failed"));
}

#[test]
fn database_with_message_creates_correct_kind() {
    let err = AppError::database_with_message("query timeout");
    assert_eq!(err.kind, AppErrorKind::Database);
    assert_eq!(err.message.as_deref(), Some("query timeout"));
}

#[test]
fn database_with_message_accepts_string() {
    let err = AppError::database_with_message("test".to_string());
    assert_eq!(err.message.as_deref(), Some("test"));
}

#[test]
fn config_constructor_creates_correct_kind() {
    let err = AppError::config("missing configuration");
    assert_eq!(err.kind, AppErrorKind::Config);
    assert_eq!(err.message.as_deref(), Some("missing configuration"));
}

#[test]
fn turnkey_constructor_creates_correct_kind() {
    let err = AppError::turnkey("turnkey error");
    assert_eq!(err.kind, AppErrorKind::Turnkey);
    assert_eq!(err.message.as_deref(), Some("turnkey error"));
}

#[test]
fn timeout_constructor_creates_correct_kind() {
    let err = AppError::timeout("operation timeout");
    assert_eq!(err.kind, AppErrorKind::Timeout);
    assert_eq!(err.message.as_deref(), Some("operation timeout"));
}

#[test]
fn network_constructor_creates_correct_kind() {
    let err = AppError::network("connection lost");
    assert_eq!(err.kind, AppErrorKind::Network);
    assert_eq!(err.message.as_deref(), Some("connection lost"));
}

#[test]
fn dependency_unavailable_constructor_creates_correct_kind() {
    let err = AppError::dependency_unavailable("service down");
    assert_eq!(err.kind, AppErrorKind::DependencyUnavailable);
    assert_eq!(err.message.as_deref(), Some("service down"));
}

#[test]
fn service_unavailable_alias_maps_to_dependency_unavailable() {
    let err = AppError::service_unavailable("unavailable");
    assert_eq!(err.kind, AppErrorKind::DependencyUnavailable);
    assert_eq!(err.message.as_deref(), Some("unavailable"));
}

#[test]
fn serialization_constructor_creates_correct_kind() {
    let err = AppError::serialization("serialize failed");
    assert_eq!(err.kind, AppErrorKind::Serialization);
    assert_eq!(err.message.as_deref(), Some("serialize failed"));
}

#[test]
fn deserialization_constructor_creates_correct_kind() {
    let err = AppError::deserialization("deserialize failed");
    assert_eq!(err.kind, AppErrorKind::Deserialization);
    assert_eq!(err.message.as_deref(), Some("deserialize failed"));
}

#[test]
fn external_api_constructor_creates_correct_kind() {
    let err = AppError::external_api("api error");
    assert_eq!(err.kind, AppErrorKind::ExternalApi);
    assert_eq!(err.message.as_deref(), Some("api error"));
}

#[test]
fn queue_constructor_creates_correct_kind() {
    let err = AppError::queue("queue full");
    assert_eq!(err.kind, AppErrorKind::Queue);
    assert_eq!(err.message.as_deref(), Some("queue full"));
}

#[test]
fn cache_constructor_creates_correct_kind() {
    let err = AppError::cache("cache miss");
    assert_eq!(err.kind, AppErrorKind::Cache);
    assert_eq!(err.message.as_deref(), Some("cache miss"));
}

#[test]
fn constructors_accept_empty_strings() {
    let err = AppError::internal("");
    assert_eq!(err.message.as_deref(), Some(""));
    let err = AppError::validation("");
    assert_eq!(err.message.as_deref(), Some(""));
}

#[test]
fn constructors_accept_unicode_messages() {
    let err = AppError::not_found("リソースが見つかりません");
    assert_eq!(err.message.as_deref(), Some("リソースが見つかりません"));
    let err = AppError::validation("Неверный ввод");
    assert_eq!(err.message.as_deref(), Some("Неверный ввод"));
}

#[test]
fn constructors_accept_long_messages() {
    let long_msg = "x".repeat(10000);
    let err = AppError::internal(long_msg.clone());
    assert_eq!(err.message.as_deref(), Some(long_msg.as_str()));
}

#[test]
fn constructors_accept_static_str() {
    const STATIC_MSG: &str = "static message";
    let err = AppError::not_found(STATIC_MSG);
    assert_eq!(err.message.as_deref(), Some(STATIC_MSG));
}
