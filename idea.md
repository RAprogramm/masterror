<!--
SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>

SPDX-License-Identifier: MIT
-->

## 1) Декларативный DSL-макрос `errors!`

Макрос сгенерирует enum, `Display`, `Error`, `MasterError`, `From`, маппинг категорий на HTTP, и билдер-конструкторы.

```rust
use masterror::prelude::*;

// comments: English
masterror::errors! {
    // Defaults for the whole enum (can be overridden per-variant)
    enum OrderError @domain("minty.api") {
        // code, category, message, fields, From/Source flags
        UnsupportedPair {
            code = 1001,
            category = InvalidInput,
            msg = "pair {pair} is not supported",
            fields = { pair: String }
        }

        RateLimited {
            code = 1002,
            category = Unavailable,
            msg = "rate limit exceeded",
            fields = { retry_after_ms: u64, #[context] ctx: Option<Context> }
        }

        IoTransparent {
            code = 2001,
            category = Internal,
            transparent,                   // delegates Display to inner
            from, source,                  // impl From<std::io::Error>
            fields = { inner: std::io::Error }
        }
    }
}
```

Что генерится:

* `pub enum OrderError { UnsupportedPair { pair: String }, ... }`
* `impl Display + Error + MasterError` с `code/domain/category/context`.
* `impl From<std::io::Error> for OrderError>` для `IoTransparent`.
* **Фабрики**: `OrderError::unsupported_pair(pair)`, `OrderError::rate_limited(retry_after_ms)`, `.with_ctx(...)`.
* `impl IntoResponse` по фиче `axum` через таблицу категорий → HTTP-статус.

### Почему это проще, чем derive построчно

* Нет ручных `#[error]`, `#[from]`, `#[source]` на каждом варианте.
* Одинаковая форма записи, меньше шансов ошибиться.
* Видны коды, категории и сообщения в одном месте как спецификация.

## 2) Конструкторы и билдер с автодополнением

```rust
let err = OrderError::unsupported_pair("BTC-FOO".to_string())
    .with_ctx(|c| c.with("req_id", "a1b2").with("user", "42"));
```

Реализация идёт через trait-расширение:

```rust
pub trait ErrorBuildExt: Sized {
    fn with_ctx<F: FnOnce(Context) -> Context>(self, f: F) -> Self;
}
```

## 3) Типобезопасные `ensure!`/`fail!` без «магии»

```rust
use masterror::prelude::*;

masterror::ensure!(
    cond = pair_is_supported(&pair),
    else = OrderError::unsupported_pair(pair.clone())
);

masterror::fail!(OrderError::rate_limited(1200));
```

Внутри это раскрывается в `return Err(...)` без аллокаций и без скрытых трюков. Никаких `panic!`, никаких глобалов.

## 4) Автонумерация кодов — но детерминированно

Макрос присвоит коды по порядку появления и зафиксирует их в сгенерированном `const`:

```rust
#[cfg(feature = "auto_codes")]
pub const ORDER_ERROR_CODES: &[(&str, u32)] = &[
    ("UnsupportedPair", 1001),
    ("RateLimited",     1002),
    ("IoTransparent",   2001),
];
```

Плюсы:

* кратко добавлять варианты,
* легко док-ген и проверка миграций.

Минус: менять порядок — меняешь коды. Лечится явной простановкой `code = ...` там, где критично.

## 5) Линтеры прямо из макроса

* пропущен `code` при выключенной `auto_codes`;
* `transparent` вместе с `msg` запрещены;
* более одного `from`/`source` в варианте;
* конфликт имён фабрик.

Сообщения макроса должны быть человекочитаемыми и точными.

## 6) Готовые мапперы для веб и телеметрии

* `axum`/`actix`: `impl IntoResponse` по `category()`, JSON-тело `{code, domain, message, context}`.
* `tracing`: helper `log_err(&E)` который пишет `error.code/domain/category` и цепочку причин.
* `utoipa`/OpenAPI: схема ошибки как `oneOf` по категориям.

Всё это подключается автоматически, если включены соответствующие фичи, кода в приложении ноль.

## 7) Шорткаты в прелюде

```rust
pub use masterror::prelude::{MasterError, Context, ErrorCategory, ensure, fail};
```

В каждом крейте приложения достаточно `use my_errors::prelude::*;`.

## 8) Интеграция с `?` и `From`

В `errors!` `from` у конкретного варианта — и только он получает `impl From<T>`. Это убирает сюрпризы и ускоряет компиляцию.

```rust
// comments: English
#[tokio::main]
async fn handler() -> Result<(), OrderError> {
    let data = std::fs::read("foo")?; // goes via IoTransparent
    Ok(())
}
```

## 9) Генерация тестов и чек-лист уникальности

Макрос сам добавляет `#[cfg(test)]` тест:

* коды уникальны,
* все `transparent`-варианты имеют ровно одно поле-source,
* `Display` не падает на формат-строках.

## 10) Миграция с thiserror за минуты

* Cуществующие enum’ы в `errors!` блоки 1:1.
* Сообщения переносятся в `msg = "... {field} ..."`.
* Для бывших `#[from]/#[source]` ставишь флаги.
* Если хочется ещё короче — включаешь `auto_codes` и не паришься до стабилизации схемы.

---

### Минимальный рабочий пример (как будет выглядеть код приложения)

```rust
use axum::{routing::get, Router};
use masterror::prelude::*;

masterror::errors! {
    enum ApiError @domain("minty.api") {
        BadParam { category = InvalidInput, msg = "bad param: {name}", fields = { name: String }, code = 1001 }
        IoTransparent { category = Internal, transparent, from, source, fields = { inner: std::io::Error }, code = 2001 }
    }
}

async fn endpoint() -> Result<&'static str, ApiError> {
    let name = std::env::var("NAME").map_err(ApiError::from)?; // IoTransparent via From<VarError>? — не даём auto From, предпочтительнее явный вариант
    masterror::ensure!(cond = !name.is_empty(), else = ApiError::bad_param(name));
    Ok("ok")
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(endpoint));
    // run...
}
```

---

## Компромиссы и почему так

* Табличный `errors!` снимает 80% рутины и делает ошибки видимой спецификацией.
* `auto_codes` удобен на старте, но фиксируй `code = ...` перед релизом.
* Макрос генерирует фабрики и билдер контекста, что убирает «лишний» конструкторный код и reduce ошибки при вызове.
* Никаких глобальных реестров, никакой «магии» с `anyhow`. Всё явно и проверяется компилятором.

