# masterror · Каркас-независимые типы ошибок приложений

> Эта страница — русская версия основной документации. Английский оригинал см. в [README.md](README.md).

[![Crates.io](https://img.shields.io/crates/v/masterror)](https://crates.io/crates/masterror)
[![docs.rs](https://img.shields.io/docsrs/masterror)](https://docs.rs/masterror)
[![Downloads](https://img.shields.io/crates/d/masterror)](https://crates.io/crates/masterror)
![MSRV](https://img.shields.io/badge/MSRV-1.90-blue)
![License](https://img.shields.io/badge/License-MIT%20or%20Apache--2.0-informational)
[![CI](https://github.com/RAprogramm/masterror/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/RAprogramm/masterror/actions/workflows/ci.yml?query=branch%3Amain)
[![Security audit](https://github.com/RAprogramm/masterror/actions/workflows/ci.yml/badge.svg?branch=main&label=Security%20audit)](https://github.com/RAprogramm/masterror/actions/workflows/ci.yml?query=branch%3Amain)
[![Cargo Deny](https://img.shields.io/github/actions/workflow/status/RAprogramm/masterror/ci.yml?branch=main&label=Cargo%20Deny)](https://github.com/RAprogramm/masterror/actions/workflows/ci.yml?query=branch%3Amain)

`masterror` вырос из набора вспомогательных функций в полноценный workspace с
модульными крейтами для построения наблюдаемых и последовательных ошибок в
Rust-сервисах. Базовый крейт остаётся независимым от веб-фреймворков, а фичи
включают только нужные адаптеры, интеграции и телеметрию. `unsafe` запрещён,
MSRV зафиксирован, а родные деривы позволяют доменным типам управлять
редактированием сообщений и метаданными.

## Ключевые возможности

- **Единая таксономия.** `AppError`, `AppErrorKind` и `AppCode` описывают
  доменные и транспортные аспекты, имеют консервативное соответствие HTTP/gRPC,
  готовые подсказки retry/auth и RFC7807-ответы через `ProblemJson`.
- **Родные деривы.** `#[derive(Error)]`, `#[derive(Masterror)]`, `#[app_error]`,
  `#[masterror(...)]` и `#[provide]` соединяют ваши типы с `AppError`, пробрасывая
  источники, бэктрейсы, телеметрию и политику редактирования.
- **Типизированная телеметрия.** `Metadata` хранит структурированные ключи и
  значения с индивидуальными правилами маскирования, а билдеры `field::*`
  избавляют от ручных `String`-карт.
- **Транспортные адаптеры.** Опциональные фичи включают респондеры для Actix/Axum,
  конвертацию в `tonic::Status`, логирование в браузер/WASM и генерацию схем
  OpenAPI без утяжеления дефолтной сборки.
- **Интеграции, проверенные в бою.** Активируйте маппинги для `sqlx`, `reqwest`,
  `redis`, `validator`, `config`, `tokio`, `teloxide`, `multipart`, Telegram
  WebApp SDK и других библиотек — каждая переводит ошибки в таксономию с
  прикреплённой телеметрией.
- **Готовые настройки.** Модуль `turnkey` поставляет готовый каталог ошибок,
  билдеры и интеграцию с `tracing` для команд, которым нужна стартовая
  конфигурация «из коробки».
- **Типобезопасные макросы управления потоком.** `ensure!` и `fail!` прерывают
  выполнение с доменной ошибкой без аллокаций и форматирования на удачной ветке.

## Состав workspace

| Крейт | Что содержит | Когда подключать |
| --- | --- | --- |
| [`masterror`](https://crates.io/crates/masterror) | Основные типы ошибок, билдеры метаданных, транспорты, интеграции и прелюдия. | Боевые сервисы и библиотеки, которым нужна стабильная поверхность ошибок. |
| [`masterror-derive`](masterror-derive/README.md) | Процедурные макросы `#[derive(Error)]`, `#[derive(Masterror)]`, `#[app_error]`, `#[provide]`. | Уже идёт транзитивно; подключайте напрямую только для экспериментов с макросами. |
| [`masterror-template`](masterror-template/README.md) | Общий парсер шаблонов для анализа форматтеров в деривах. | Внутренний компонент; используйте, если нужен этот парсер в другом коде. |

## Флаги фич

Все фичи отключены по умолчанию — выбирайте только нужное.

- **Веб и API:** `axum`, `actix`, `multipart`, `openapi`, `serde_json`.
- **Наблюдаемость:** `tracing`, `metrics`, `backtrace`.
- **Async и IO:** `tokio`, `reqwest`, `sqlx`, `sqlx-migrate`, `redis`, `validator`,
  `config`.
- **Боты и мессенджеры:** `teloxide`, `telegram-webapp-sdk`.
- **Фронтенд:** `frontend` для логирования в браузере/WASM.
- **gRPC:** `tonic` для генерации `tonic::Status`.
- **Готовая таксономия:** `turnkey`.

## TL;DR

~~~toml
[dependencies]
masterror = { version = "0.21.1", default-features = false }
# или с нужными фичами:
# masterror = { version = "0.21.1", features = [
#   "axum", "actix", "openapi", "serde_json",
#   "tracing", "metrics", "backtrace", "sqlx",
#   "sqlx-migrate", "reqwest", "redis", "validator",
#   "config", "tokio", "multipart", "teloxide",
#   "telegram-webapp-sdk", "tonic", "frontend", "turnkey"
# ] }
~~~

---

### Быстрый старт

Создание ошибки вручную:

~~~rust
use masterror::{AppError, AppErrorKind, field};

let err = AppError::new(AppErrorKind::BadRequest, "Флаг должен быть установлен");
assert!(matches!(err.kind, AppErrorKind::BadRequest));
let err_with_meta = AppError::service("downstream")
    .with_field(field::str("request_id", "abc123"));
assert_eq!(err_with_meta.metadata().len(), 1);
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

### Макросы для раннего возврата без потери типизации

`ensure!` и `fail!` — типизированные аналоги `anyhow::ensure!`/`anyhow::bail!`.
Они вычисляют выражение ошибки только при срабатывании гварда, поэтому
успешный путь остаётся без аллокаций.

~~~rust
use masterror::{AppError, AppErrorKind, AppResult};

fn guard(flag: bool) -> AppResult<()> {
    masterror::ensure!(flag, AppError::bad_request("Флаг обязателен"));
    Ok(())
}

fn bail() -> AppResult<()> {
    masterror::fail!(AppError::unauthorized("Токен истёк"));
}

assert!(guard(true).is_ok());
assert!(matches!(guard(false).unwrap_err().kind, AppErrorKind::BadRequest));
assert!(matches!(bail().unwrap_err().kind, AppErrorKind::Unauthorized));
~~~

### Деривы для доменных ошибок и транспорта

`masterror` предоставляет родные деривы, чтобы типы оставались выразительными, а
crate отвечал за конверсии, телеметрию и редактирование сообщений.

~~~rust
use std::io;

use masterror::Error;

#[derive(Debug, Error)]
#[error("I/O failed: {source}")]
pub struct DomainError {
    #[from]
    #[source]
    source: io::Error,
}

#[derive(Debug, Error)]
#[error(transparent)]
pub struct WrappedDomainError(
    #[from]
    #[source]
    DomainError
);

fn load() -> Result<(), DomainError> {
    Err(io::Error::other("disk offline").into())
}

let err = load().unwrap_err();
assert_eq!(err.to_string(), "I/O failed: disk offline");

let wrapped = WrappedDomainError::from(err);
assert_eq!(wrapped.to_string(), "I/O failed: disk offline");
~~~

- `use masterror::Error;` подключает макрос дерива.
- `#[from]` автоматически реализует `From<...>` и проверяет форму враппера.
- `#[error(transparent)]` гарантирует корректную прокладку `Display`/`source`.
- `#[app_error(kind = ..., code = ..., message)]` сопоставляет ошибку с
  `AppError`/`AppCode`; `code = ...` добавляет `From<Error> for AppCode`, а
  `message` публикует форматированную строку вместо обезличенного текста.
- `masterror::error::template::ErrorTemplate` разбирает строки формата, позволяя
  строить собственные деривы без зависимости от `thiserror`.

### Телеметрия, редактирование и маппинги транспортов

`#[derive(Masterror)]` преобразует доменную ошибку в [`masterror::Error`],
прикрепляя метаданные, политику редактирования и маппинги для HTTP/gRPC/RFC7807.

~~~rust
use masterror::{
    mapping::HttpMapping, AppCode, AppErrorKind, Error, Masterror, MessageEditPolicy
};

#[derive(Debug, Masterror)]
#[error("user {user_id} missing flag {flag}")]
#[masterror(
    code = AppCode::NotFound,
    category = AppErrorKind::NotFound,
    message,
    redact(message, fields("user_id" = hash)),
    telemetry(
        Some(masterror::field::str("user_id", user_id.clone())),
        attempt.map(|value| masterror::field::u64("attempt", value))
    ),
    map.grpc = 5,
    map.problem = "https://errors.example.com/not-found"
)]
struct MissingFlag {
    user_id: String,
    flag: &'static str,
    attempt: Option<u64>,
    #[source]
    source: Option<std::io::Error>
}

let err = MissingFlag {
    user_id: "alice".into(),
    flag: "beta",
    attempt: Some(2),
    source: None
};
let converted: Error = err.into();
assert_eq!(converted.code, AppCode::NotFound);
assert_eq!(converted.kind, AppErrorKind::NotFound);
assert_eq!(converted.edit_policy, MessageEditPolicy::Redact);
assert!(converted.metadata().get("user_id").is_some());

assert_eq!(
    MissingFlag::HTTP_MAPPING,
    HttpMapping::new(AppCode::NotFound, AppErrorKind::NotFound)
);
~~~

- `code` / `category` задают публичный [`AppCode`] и внутренний
  [`AppErrorKind`].
- `message` публикует форматированную строку как безопасное сообщение.
- `redact(message)` включает редактирование, а `fields("name" = hash)` задаёт
  правила маскирования для метаданных.
- `telemetry(...)` принимает выражения, дающие `Option<Field>`; заполненные поля
  попадают в [`Metadata`].
- `map.grpc` / `map.problem` добавляют gRPC-код и RFC7807 `type` URI. Дерив
  генерирует таблицы `HTTP_MAPPING`, `GRPC_MAPPING`, `PROBLEM_MAPPING`.

Все атрибуты уровня полей (`#[from]`, `#[source]`, `#[backtrace]`) продолжают
работать. Источники и бэктрейсы автоматически прикрепляются к
[`masterror::Error`].

### Провайдеры телеметрии и `AppError`

`#[provide(...)]` раскрывает структурированную телеметрию через
`std::error::Request`, а `#[app_error(...)]` описывает конверсию в `AppError` и
`AppCode`.

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

Опциональная телеметрия не регистрирует провайдер, если значение `None`, а
владение можно передать через `value = ...`.

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

Перечисления поддерживают собственные маппинги и провайдеры на вариант:

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

Так вы сохраняете знакомый интерфейс `thiserror`, но получаете телеметрию и
готовые конверсии в `AppError`/`AppCode` без ручного кода.

### Problem JSON и подсказки retry/auth

~~~rust
use masterror::{AppError, AppErrorKind, ProblemJson};
use std::time::Duration;

let problem = ProblemJson::from_app_error(
    AppError::new(AppErrorKind::Unauthorized, "Token expired")
        .with_retry_after_duration(Duration::from_secs(30))
        .with_www_authenticate(r#"Bearer realm="api", error="invalid_token""#)
);

assert_eq!(problem.status, 401);
assert_eq!(problem.retry_after, Some(30));
assert_eq!(problem.grpc.expect("grpc").name, "UNAUTHENTICATED");
~~~

### Дополнительные материалы

- [Вики по обработке ошибок](docs/wiki/index.md) с пошаговыми руководствами,
  сравнением `thiserror`/`anyhow` и рецептами устранения проблем.
- [Документация на docs.rs](https://docs.rs/masterror) с подробными таблицами по
  фичам и транспортам.
- [`CHANGELOG.md`](CHANGELOG.md) для истории релизов и миграций.

---

MSRV: **1.90** · Лицензия: **MIT OR Apache-2.0** · Без `unsafe`
