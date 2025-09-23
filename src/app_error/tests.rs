use std::{borrow::Cow, error::Error as StdError, fmt::Display, sync::Arc};
#[cfg(feature = "backtrace")]
use std::{env, sync::Mutex};

#[cfg(feature = "backtrace")]
use super::core::reset_backtrace_preference;

#[cfg(feature = "backtrace")]
static BACKTRACE_ENV_GUARD: Mutex<()> = Mutex::new(());

use super::{AppError, FieldRedaction, FieldValue, MessageEditPolicy, field};
use crate::{AppCode, AppErrorKind};

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
fn retry_and_www_authenticate_are_attached() {
    let err = AppError::internal("boom")
        .with_retry_after_secs(30)
        .with_www_authenticate("Bearer");
    assert_eq!(err.retry.unwrap().after_seconds, 30);
    assert_eq!(err.www_authenticate.as_deref(), Some("Bearer"));
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
fn with_backtrace_env<F: FnOnce()>(value: Option<&str>, test: F) {
    let _guard = BACKTRACE_ENV_GUARD.lock().expect("env guard");
    reset_backtrace_preference();
    match value {
        Some(val) => env::set_var("RUST_BACKTRACE", val),
        None => env::remove_var("RUST_BACKTRACE")
    }
    test();
    env::remove_var("RUST_BACKTRACE");
    reset_backtrace_preference();
}

#[cfg(feature = "backtrace")]
#[test]
fn backtrace_respects_disabled_env() {
    with_backtrace_env(Some("0"), || {
        let err = AppError::internal("boom");
        assert!(err.backtrace().is_none());
    });
}

#[cfg(feature = "backtrace")]
#[test]
fn backtrace_enabled_when_env_requests() {
    with_backtrace_env(Some("1"), || {
        let err = AppError::internal("boom");
        assert!(err.backtrace().is_some());
    });
}

#[test]
fn redactable_policy_is_exposed() {
    let err = AppError::internal("boom").redactable();
    assert!(matches!(err.edit_policy, MessageEditPolicy::Redact));
}

#[test]
fn log_uses_kind_and_code() {
    // Smoke test to ensure the method is callable; tracing output isn't asserted
    // here.
    let err = AppError::internal("boom");
    err.log();
}

#[cfg(feature = "tracing")]
#[test]
fn telemetry_emits_single_tracing_event_with_trace_id() {
    use std::{
        fmt,
        sync::{Arc, Mutex}
    };

    use tracing::{
        Dispatch, Event, Subscriber, dispatcher,
        field::{Field, Visit}
    };
    use tracing_subscriber::{
        Registry,
        layer::{Context, Layer, SubscriberExt}
    };

    #[derive(Default, Clone)]
    struct RecordedEvent {
        trace_id: Option<String>,
        code:     Option<String>,
        category: Option<String>
    }

    struct RecordingLayer {
        events: Arc<Mutex<Vec<RecordedEvent>>>
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

    let events = Arc::new(Mutex::new(Vec::new()));
    let layer = RecordingLayer {
        events: events.clone()
    };
    let subscriber = Registry::default().with(layer);
    let dispatch = Dispatch::new(subscriber);

    dispatcher::with_default(&dispatch, || {
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
    fn app() -> super::AppResult<u8> {
        Ok(1)
    }

    fn other() -> super::AppResult<u8, &'static str> {
        Ok(2)
    }

    assert_eq!(app().unwrap(), 1);
    assert_eq!(other().unwrap(), 2);
}
