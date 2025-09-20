# masterror · Каркас-независимые типы ошибок приложений

> Этот документ — русская версия основной документации. Английскую версию см. в [README.md](README.md).

[![Crates.io](https://img.shields.io/crates/v/masterror)](https://crates.io/crates/masterror)
[![docs.rs](https://img.shields.io/docsrs/masterror)](https://docs.rs/masterror)
[![Downloads](https://img.shields.io/crates/d/masterror)](https://crates.io/crates/masterror)
![MSRV](https://img.shields.io/badge/MSRV-1.90-blue)
![License](https://img.shields.io/badge/License-MIT%20or%20Apache--2.0-informational)
[![CI](https://github.com/RAprogramm/masterror/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/RAprogramm/masterror/actions/workflows/ci.yml?query=branch%3Amain)
[![Security audit](https://github.com/RAprogramm/masterror/actions/workflows/ci.yml/badge.svg?branch=main&label=Security%20audit)](https://github.com/RAprogramm/masterror/actions/workflows/ci.yml?query=branch%3Amain)
[![Cargo Deny](https://img.shields.io/github/actions/workflow/status/RAprogramm/masterror/ci.yml?branch=main&label=Cargo%20Deny)](https://github.com/RAprogramm/masterror/actions/workflows/ci.yml?query=branch%3Amain)

Небольшая прагматичная модель ошибок для Rust-сервисов с выраженным API. Основной крейт не зависит от веб-фреймворков, а расширения включаются через фичи. Таксономия ошибок стабильна, соответствие HTTP-кодам консервативно, `unsafe` запрещён.

## Основные возможности

- Базовые типы: `AppError`, `AppErrorKind`, `AppResult`, `AppCode`, `ErrorResponse`.
- Адаптеры для Axum и Actix (опционально).
- Генерация схем OpenAPI через `utoipa`.
- Конверсии из распространённых библиотек (`sqlx`, `reqwest`, `redis`, `validator`, `config`, `tokio` и др.).
- Готовый прелюдия-модуль, реэкспортирующий наиболее востребованные типы и трейты.

## Установка

Добавьте зависимость в `Cargo.toml`:

~~~toml
[dependencies]
# минимальное ядро
masterror = { version = "0.10.3", default-features = false }
# или с нужными интеграциями
# masterror = { version = "0.10.3", features = [
#   "axum", "actix", "openapi", "serde_json",
#   "sqlx", "sqlx-migrate", "reqwest", "redis",
#   "validator", "config", "tokio", "multipart",
#   "teloxide", "telegram-webapp-sdk", "frontend", "turnkey"
# ] }
~~~

**MSRV:** 1.90

## Быстрый старт

Создание ошибки вручную:

~~~rust
use masterror::{AppError, AppErrorKind};

let err = AppError::new(AppErrorKind::BadRequest, "Флаг должен быть установлен");
assert!(matches!(err.kind, AppErrorKind::BadRequest));
~~~

Использование прелюдии:

~~~rust
use masterror::prelude::*;

fn do_work(flag: bool) -> AppResult<()> {
    if !flag {
        return Err(AppError::bad_request("Флаг должен быть установлен"));
    }
    Ok(())
}
~~~

## Дополнительные интеграции

- `sqlx` — классификация `sqlx::Error` по видам ошибок.
- `sqlx-migrate` — обработка `sqlx::migrate::MigrateError` как базы данных.
- `reqwest` — перевод сетевых/HTTP-сбоев в доменные категории.
- `redis` — корректная обработка ошибок кеша.
- `validator` — преобразование `ValidationErrors` в валидационные ошибки API.
- `config` — типизированные ошибки конфигурации.
- `tokio` — маппинг таймаутов (`tokio::time::error::Elapsed`).
- `multipart` — обработка ошибок извлечения multipart в Axum.
- `teloxide` — маппинг `teloxide_core::RequestError` в доменные категории.
- `telegram-webapp-sdk` — обработка ошибок валидации данных Telegram WebApp.
- `frontend` — логирование в браузере и преобразование в `JsValue` для WASM.
- `turnkey` — расширение таксономии для Turnkey SDK.

## Атрибуты `#[provide]` и `#[app_error]`

Атрибут `#[provide(...)]` позволяет передавать структурированную телеметрию через
`std::error::Request`, а `#[app_error(...)]` описывает прямой маппинг доменной
ошибки в `AppError` и `AppCode`. Дерив сохраняет синтаксис `thiserror`, но
дополняет его провайдерами телеметрии и готовыми конверсиями в типы `masterror`.

~~~rust
use std::error::request_ref;

use masterror::{AppCode, AppError, AppErrorKind, Error};

#[derive(Clone, Debug, PartialEq, Eq)]
struct TelemetrySnapshot {
    name:  &'static str,
    value: u64,
}

#[derive(Debug, Error)]
#[error("structured telemetry {snapshot:?}")]
#[app_error(kind = AppErrorKind::Service, code = AppCode::Service)]
struct StructuredTelemetryError {
    #[provide(ref = TelemetrySnapshot, value = TelemetrySnapshot)]
    snapshot: TelemetrySnapshot,
}

let err = StructuredTelemetryError {
    snapshot: TelemetrySnapshot {
        name: "db.query",
        value: 42,
    },
};

let snapshot = request_ref::<TelemetrySnapshot>(&err).expect("telemetry");
assert_eq!(snapshot.value, 42);

let app: AppError = err.into();
let via_app = request_ref::<TelemetrySnapshot>(&app).expect("telemetry");
assert_eq!(via_app.name, "db.query");
~~~

Опциональные поля автоматически пропускаются, если значения нет. При запросе
значения `Option<T>` можно вернуть как по ссылке, так и передать владение:

~~~rust
use masterror::{AppCode, AppErrorKind, Error};

#[derive(Debug, Error)]
#[error("optional telemetry {telemetry:?}")]
#[app_error(kind = AppErrorKind::Internal, code = AppCode::Internal)]
struct OptionalTelemetryError {
    #[provide(ref = TelemetrySnapshot, value = TelemetrySnapshot)]
    telemetry: Option<TelemetrySnapshot>,
}

let noisy = OptionalTelemetryError {
    telemetry: Some(TelemetrySnapshot {
        name: "queue.depth",
        value: 17,
    }),
};
let silent = OptionalTelemetryError { telemetry: None };

assert!(request_ref::<TelemetrySnapshot>(&noisy).is_some());
assert!(request_ref::<TelemetrySnapshot>(&silent).is_none());
~~~

Для перечислений каждая ветка может задавать собственную телеметрию и
конверсию. Дерив сгенерирует единый `From<Enum>` для `AppError`/`AppCode`:

~~~rust
#[derive(Debug, Error)]
enum EnumTelemetryError {
    #[error("named {label}")]
    #[app_error(kind = AppErrorKind::NotFound, code = AppCode::NotFound)]
    Named {
        label:    &'static str,
        #[provide(ref = TelemetrySnapshot)]
        snapshot: TelemetrySnapshot,
    },
    #[error("optional tuple")]
    #[app_error(kind = AppErrorKind::Timeout, code = AppCode::Timeout)]
    Optional(#[provide(ref = TelemetrySnapshot)] Option<TelemetrySnapshot>),
    #[error("owned tuple")]
    #[app_error(kind = AppErrorKind::Service, code = AppCode::Service)]
    Owned(#[provide(value = TelemetrySnapshot)] TelemetrySnapshot),
}

let owned = EnumTelemetryError::Owned(TelemetrySnapshot {
    name: "redis.latency",
    value: 3,
});
let app: AppError = owned.into();
assert!(matches!(app.kind, AppErrorKind::Service));
~~~

В отличие от `thiserror`, вы получаете дополнительную структурированную
информацию и прямой маппинг в `AppError`/`AppCode` без ручных реализаций
`From`.

## Форматирование шаблонов `#[error]`

Шаблон `#[error("...")]` по умолчанию использует `Display`, но любая
подстановка может запросить другой форматтер.
`TemplateFormatter::is_alternate()` фиксирует флаг `#`, а `TemplateFormatterKind`
сообщает, какой трейт `core::fmt` нужен, поэтому порождённый код может
переключаться между вариантами без ручного `match`. Неподдержанные спецификаторы
приводят к диагностике на этапе компиляции, совпадающей с `thiserror`.

| Спецификатор     | Трейт                   | Пример результата        | Примечания |
|------------------|-------------------------|--------------------------|------------|
| _по умолчанию_   | `core::fmt::Display`    | `value`                  | Пользовательские сообщения; `#` игнорируется. |
| `:?` / `:#?`     | `core::fmt::Debug`      | `Struct { .. }` / многострочный | Поведение `Debug`; `#` включает pretty-print. |
| `:x` / `:#x`     | `core::fmt::LowerHex`   | `0x2a`                   | Шестнадцатеричный вывод; `#` добавляет `0x`. |
| `:X` / `:#X`     | `core::fmt::UpperHex`   | `0x2A`                   | Верхний регистр; `#` добавляет `0x`. |
| `:p` / `:#p`     | `core::fmt::Pointer`    | `0x1f00` / `0x1f00`      | Сырые указатели; `#` поддерживается для совместимости. |
| `:b` / `:#b`     | `core::fmt::Binary`     | `101010` / `0b101010`   | Двоичный вывод; `#` добавляет `0b`. |
| `:o` / `:#o`     | `core::fmt::Octal`      | `52` / `0o52`           | Восьмеричный вывод; `#` добавляет `0o`. |
| `:e` / `:#e`     | `core::fmt::LowerExp`   | `1.5e-2`                | Научная запись; `#` заставляет выводить десятичную точку. |
| `:E` / `:#E`     | `core::fmt::UpperExp`   | `1.5E-2`                | Верхний регистр научной записи; `#` заставляет выводить точку. |

- `TemplateFormatterKind::supports_alternate()` сообщает, имеет ли смысл `#` для
  выбранного трейта (для указателей вывод совпадает с обычным).
- `TemplateFormatterKind::specifier()` возвращает канонический символ
  спецификатора, что упрощает повторный рендеринг плейсхолдеров.
- `TemplateFormatter::from_kind(kind, alternate)` собирает форматтер из
  `TemplateFormatterKind`, позволяя программно переключать флаг `#`.
- Display-плейсхолдеры сохраняют исходные параметры форматирования:
  методы `TemplateFormatter::display_spec()` и
  `TemplateFormatter::format_fragment()` возвращают `:>8`, `:.3` и другие
  варианты без необходимости собирать строку вручную.

~~~rust
use core::ptr;

use masterror::Error;

#[derive(Debug, Error)]
#[error(
    "debug={payload:?}, hex={id:#x}, ptr={ptr:p}, bin={mask:#b}, \
     oct={mask:o}, lower={ratio:e}, upper={ratio:E}"
)]
struct FormatterDemo {
    id: u32,
    payload: String,
    ptr: *const u8,
    mask: u8,
    ratio: f32,
}

let err = FormatterDemo {
    id: 0x2a,
    payload: "hello".into(),
    ptr: ptr::null(),
    mask: 0b1010_0001,
    ratio: 0.15625,
};

let rendered = err.to_string();
assert!(rendered.contains("debug=\"hello\""));
assert!(rendered.contains("hex=0x2a"));
assert!(rendered.contains("ptr=0x0"));
assert!(rendered.contains("bin=0b10100001"));
assert!(rendered.contains("oct=241"));
assert!(rendered.contains("lower=1.5625e-1"));
assert!(rendered.contains("upper=1.5625E-1"));
~~~

`masterror::error::template::ErrorTemplate` позволяет разобрать шаблон и
программно проверить запрошенные форматтеры; перечисление
`TemplateFormatterKind` возвращает название трейта для каждого плейсхолдера:

~~~rust
use masterror::error::template::{
    ErrorTemplate, TemplateFormatter, TemplateFormatterKind
};

let template = ErrorTemplate::parse("{code:#x} → {payload:?}").expect("parse");
let mut placeholders = template.placeholders();

let code = placeholders.next().expect("code placeholder");
let code_formatter = code.formatter();
assert!(matches!(
    code_formatter,
    TemplateFormatter::LowerHex { alternate: true }
));
let code_kind = code_formatter.kind();
assert_eq!(code_kind, TemplateFormatterKind::LowerHex);
assert!(code_formatter.is_alternate());
assert_eq!(code_kind.specifier(), Some('x'));
assert!(code_kind.supports_alternate());
let lowered = TemplateFormatter::from_kind(code_kind, false);
assert!(matches!(
    lowered,
    TemplateFormatter::LowerHex { alternate: false }
));

let payload = placeholders.next().expect("payload placeholder");
let payload_formatter = payload.formatter();
assert_eq!(
    payload_formatter,
    &TemplateFormatter::Debug { alternate: false }
);
let payload_kind = payload_formatter.kind();
assert_eq!(payload_kind, TemplateFormatterKind::Debug);
assert_eq!(payload_kind.specifier(), Some('?'));
assert!(payload_kind.supports_alternate());
let pretty_debug = TemplateFormatter::from_kind(payload_kind, true);
assert!(matches!(
    pretty_debug,
    TemplateFormatter::Debug { alternate: true }
));
assert!(pretty_debug.is_alternate());
~~~

Опции выравнивания, точности и заполнения для `Display` сохраняются и доступны
для прямой передачи в `write!`:

~~~rust
use masterror::error::template::ErrorTemplate;

let aligned = ErrorTemplate::parse("{value:>8}").expect("parse");
let display = aligned.placeholders().next().expect("display placeholder");
assert_eq!(display.formatter().display_spec(), Some(">8"));
assert_eq!(
    display
        .formatter()
        .format_fragment()
        .as_deref(),
    Some(">8")
);
~~~

Динамические ширина и точность (`{value:>width$}`, `{value:.precision$}`)
тоже доходят до вызова `write!`, если объявить соответствующие аргументы в
атрибуте `#[error(...)]`:

~~~rust
use masterror::Error;

#[derive(Debug, Error)]
#[error("{value:>width$}", value = .value, width = .width)]
struct DynamicWidth {
    value: &'static str,
    width: usize,
}

#[derive(Debug, Error)]
#[error("{value:.precision$}", value = .value, precision = .precision)]
struct DynamicPrecision {
    value: f64,
    precision: usize,
}

let width = DynamicWidth {
    value: "x",
    width: 5,
};
let precision = DynamicPrecision {
    value: 123.456_f64,
    precision: 4,
};

assert_eq!(width.to_string(), format!("{value:>width$}", value = "x", width = 5));
assert_eq!(
    precision.to_string(),
    format!("{value:.precision$}", value = 123.456_f64, precision = 4)
);
~~~

> **Совместимость с `thiserror` v2.** Доступные спецификаторы, сообщения об
> ошибках и поведение совпадают с `thiserror` 2.x, поэтому миграция с
> `thiserror::Error` на `masterror::Error` не требует переписывать шаблоны.

## Лицензия

Проект распространяется по лицензии Apache-2.0 или MIT на ваш выбор.
