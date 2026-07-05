# Поддержка no_std

`masterror` собирается без стандартной библиотеки Rust. Корень крейта
объявляет `#![cfg_attr(not(feature = "std"), no_std)]`, и включённая по
умолчанию функция `std` — единственное, что отделяет вас от сборки, дружелюбной
к embedded/WASM:

```toml
[dependencies]
masterror = { version = "0.28", default-features = false }
```

## alloc обязателен

`masterror` — это `no_std`, но **не** `no_alloc`. Крейт безусловно объявляет
`extern crate alloc` и использует `Cow<'static, str>`, `String`, `Arc` и
`BTreeMap` для сообщений, метаданных и цепочек источников. Вашей целевой
платформе нужен глобальный аллокатор; окружения только с `core` не
поддерживаются.

## Что работает без `std`

Всё фреймворк-независимое ядро:

| Область | Доступно в `no_std` |
|---|---|
| Основные типы | `Error` / `AppError`, `AppResult`, `AppErrorKind`, `AppCode` |
| Метаданные | `Metadata`, `Field`, `FieldValue`, `FieldRedaction`, хелперы `field::*` |
| Контекст | `Context`, `ResultExt::{ctx, context}` |
| Управление потоком | `ensure!`, `fail!` |
| Derive | `#[derive(Error)]`, `#[derive(Masterror)]` со всеми атрибутами |
| Типы на проводе | `ProblemJson`, `ErrorResponse`, `CODE_MAPPINGS`, `mapping_for_code` |
| Интроспекция | `chain()`, `root_cause()`, `is`/`downcast`/`downcast_ref`/`downcast_mut`, `render_message()` |
| Serde | `serde` с `alloc` (JSON-сериализация типов на проводе) |

Источники ошибок работают через **`core::error::Error`**: крейт реализует и
использует `core::error::Error` (внутренний псевдоним `CoreError`) вместо
`std::error::Error`, поэтому `with_source(...)`, цепочки источников и
даункастинг полностью функциональны в сборках `no_std`.

```rust
use masterror::{AppCode, AppError, AppErrorKind, field};

let err = AppError::new(AppErrorKind::Timeout, "deadline exceeded")
    .with_field(field::u64("attempt", 3));

assert_eq!(err.code, AppCode::Timeout);
assert_eq!(err.metadata().len(), 1);
```

## Что требует `std`

Каждая runtime-интеграция явно включает `std` обратно в определении своей
функции. Из `Cargo.toml`:

- `tracing`, `metrics`, `backtrace`, `colored`
- `axum`, `actix`, `multipart`, `tonic`, `openapi`
- `serde_json`, `redis`, `validator`, `config`, `tokio`, `reqwest`,
  `teloxide`, `init-data`, `frontend`, `turnkey`

`backtrace` нуждается в `std::backtrace::Backtrace` и доступе к окружению;
`colored` — в детекции TTY; веб- и клиентским интеграциям нужны их хост-крейты,
которые сами работают только со `std`.

## Матрица функций в CI

CI-задача `no_std` (`.github/workflows/ci.yml`) проверяет эти комбинации на
каждом pull request и push в `main`:

| Задача | Команда | Что проверяет |
|---|---|---|
| `bare` | `cargo check --no-default-features` | настоящую сборку `no_std` + `alloc` |
| `std-only` | `cargo check --features std` | стандартную поверхность std |
| `tracing` | `cargo check --no-default-features --features tracing` | что одиночная телеметрическая функция собирается автономно |
| `metrics` | `cargo check --no-default-features --features metrics` | то же для metrics |
| `colored` | `cargo check --no-default-features --features colored` | то же для colored |
| `all-features` | `cargo check --all-features` | полное объединение функций |

Обратите внимание на семантику: только задача `bare` — настоящая компиляция
`no_std`. `tracing = [..., "std"]`, `metrics = [..., "std"]` и
`colored = [..., "std"]` транзитивно включают `std` обратно, поэтому эти
задачи проверяют, что каждая телеметрическая функция самодостаточна при
отключённых значениях по умолчанию — а не то, что телеметрия работает без
стандартной библиотеки. Если нужна телеметрия — нужен `std`.

## Практическая настройка

Библиотечные крейты, которые хотят оставаться транспортно-независимыми и
совместимыми с `no_std`:

```toml
[dependencies]
masterror = { version = "0.28", default-features = false }

[features]
std = ["masterror/std"]
```

Бинарный или сервисный крейт затем включает `std` плюс нужные ему интеграции:

```toml
[dependencies]
masterror = { version = "0.28", features = ["axum", "tracing", "metrics"] }
```

Поскольку `AppErrorKind`, `AppCode` и типы на проводе живут в ядре `no_std`,
доменные крейты могут классифицировать ошибки и даже строить полезные нагрузки
`ProblemJson`, тогда как отображение на HTTP происходит только в сервисном
крейте — см. [Лучшие практики](Лучшие-практики).

## Тулчейн

Крейт нацелен на edition 2024 с `rust-version = "1.96"` в `Cargo.toml`.
`core::error::Error` (основа цепочек источников в `no_std`) стабилен начиная с
Rust 1.81, поэтому никакие nightly-функции не задействованы.

См. также: [Флаги возможностей](Флаги-возможностей) · [Начало работы](Начало-работы) · [Лучшие практики](Лучшие-практики)
