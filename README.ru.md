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
masterror = { version = "0.5.7", default-features = false }
# или с нужными интеграциями
# masterror = { version = "0.5.7", features = [
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

## Форматирование шаблонов `#[error]`

Шаблон `#[error("...")]` по умолчанию использует `Display`, но любая
подстановка может запросить другой форматтер. `masterror::Error` понимает тот же
набор спецификаторов, что и `thiserror` v2: `:?`, `:x`, `:X`, `:p`, `:b`, `:o`,
`:e`, `:E`, а также их версии с `#` для альтернативного вывода. Неподдержанные
форматтеры приводят к диагностике на этапе компиляции, совпадающей с
`thiserror`.

| Спецификатор     | Трейт                   | Пример результата        |
|------------------|-------------------------|--------------------------|
| _по умолчанию_   | `core::fmt::Display`    | `value`                  |
| `:?` / `:#?`     | `core::fmt::Debug`      | `Struct { .. }` / многострочный |
| `:x` / `:#x`     | `core::fmt::LowerHex`   | `0x2a`                   |
| `:X` / `:#X`     | `core::fmt::UpperHex`   | `0x2A`                   |
| `:p` / `:#p`     | `core::fmt::Pointer`    | `0x1f00` / `0x1f00`      |
| `:b` / `:#b`     | `core::fmt::Binary`     | `101010` / `0b101010`   |
| `:o` / `:#o`     | `core::fmt::Octal`      | `52` / `0o52`           |
| `:e` / `:#e`     | `core::fmt::LowerExp`   | `1.5e-2`                |
| `:E` / `:#E`     | `core::fmt::UpperExp`   | `1.5E-2`                |

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
программно проверить запрошенные форматтеры:

~~~rust
use masterror::error::template::{ErrorTemplate, TemplateFormatter};

let template = ErrorTemplate::parse("{code:#x} → {payload:?}").expect("parse");
let mut placeholders = template.placeholders();

let code = placeholders.next().expect("code placeholder");
assert!(matches!(
    code.formatter(),
    TemplateFormatter::LowerHex { alternate: true }
));

let payload = placeholders.next().expect("payload placeholder");
assert_eq!(
    payload.formatter(),
    TemplateFormatter::Debug { alternate: false }
);
~~~

> **Совместимость с `thiserror` v2.** Доступные спецификаторы, сообщения об
> ошибках и поведение совпадают с `thiserror` 2.x, поэтому миграция с
> `thiserror::Error` на `masterror::Error` не требует переписывать шаблоны.

## Лицензия

Проект распространяется по лицензии Apache-2.0 или MIT на ваш выбор.
