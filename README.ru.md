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
masterror = { version = "0.5.2", default-features = false }
# или с нужными интеграциями
# masterror = { version = "0.5.2", features = [
#   "axum", "actix", "openapi", "serde_json",
#   "sqlx", "reqwest", "redis", "validator",
#   "config", "tokio", "multipart", "teloxide",
#   "telegram-webapp-sdk", "frontend", "turnkey"
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
- `reqwest` — перевод сетевых/HTTP-сбоев в доменные категории.
- `redis` — корректная обработка ошибок кеша.
- `validator` — преобразование `ValidationErrors` в валидационные ошибки API.
- `config` — типизированные ошибки конфигурации.
- `tokio` — маппинг таймаутов (`tokio::time::error::Elapsed`).
- `multipart` — обработка ошибок извлечения multipart в Axum.
- `frontend` — логирование в браузере и преобразование в `JsValue` для WASM.
- `turnkey` — расширение таксономии для Turnkey SDK.

## Лицензия

Проект распространяется по лицензии Apache-2.0 или MIT на ваш выбор.
