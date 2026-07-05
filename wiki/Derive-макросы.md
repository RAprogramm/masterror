# Derive-макросы

`masterror` поставляет два derive-макроса через прилагаемый крейт `masterror-derive`:

- **`#[derive(Error)]`** — прямая замена `thiserror::Error` (те же атрибуты `#[error]`, `#[from]`, `#[source]`, `#[backtrace]`), расширенная конверсиями `#[app_error(...)]` и телеметрией `#[provide(...)]`.
- **`#[derive(Masterror)]`** — строится на том же синтаксисе и встраивает доменную ошибку напрямую в `masterror::Error` с метаданными, политикой редактирования и таблицами транспортных отображений через `#[masterror(...)]`.

Оба реэкспортируются из корня: `use masterror::{Error, Masterror};`.

## Шаблоны `#[error("...")]`

Шаблон определяет генерируемую реализацию `Display`. Плейсхолдеры ссылаются на поля по имени (`{field}`), индексу кортежа (`{0}`) или через явные аргументы. Разбор выполняет общий крейт `masterror-template`, семантика повторяет `thiserror`.

```rust
use masterror::Error;

#[derive(Debug, Error)]
#[error("{kind}: {message}")]
struct NamedError {
    kind:    &'static str,
    message: &'static str
}

#[derive(Debug, Error)]
#[error("{0} -> {1:?}")]
struct TupleError(&'static str, u8);
```

### Трейты форматирования и спецификаторы

Плейсхолдеры поддерживают полный набор форматтеров — `{x:?}`, `{x:#?}`, `{x:x}`, `{x:#X}`, `{x:b}`, `{x:o}`, `{x:e}`, `{x:E}`, `{x:p}` — а display-спецификаторы вроде `{value:>8}` или `{ratio:.3}` пробрасываются как есть. Для программного анализа шаблонов `masterror::error::template` предоставляет `ErrorTemplate`, `TemplateFormatter` и `TemplateFormatterKind`.

### Аргументы форматирования и проекции

Шаблоны принимают именованные и позиционные аргументы, включая выражения над `self` и проекции полей через сокращение `.field`:

```rust
use masterror::Error;

#[derive(Debug, Error)]
#[error("{formatted}", formatted = self.message.to_uppercase())]
struct FormatArgExpressionError {
    message: &'static str
}

#[derive(Debug, Error)]
#[error("{}, {label}, {}", label = self.label, self.first, self.second)]
struct MixedImplicitArgsError {
    label:  &'static str,
    first:  &'static str,
    second: &'static str
}

#[derive(Debug, Error)]
#[error("{value}", value = .value)]
struct FieldShortcutError {
    value: &'static str
}
```

### `transparent` и `fmt = ...`

```rust
use masterror::Error;

#[derive(Debug, Error)]
#[error("inner failure")]
struct Inner;

// Forwards Display and source() to the single wrapped field
#[derive(Debug, Error)]
#[error(transparent)]
struct Wrapper(#[from] Inner);

// Delegate rendering to a function: fields first, formatter last
fn render(count: &usize, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    write!(f, "count={count}")
}

#[derive(Debug, Error)]
#[error(fmt = crate::render)]
struct CustomFormat {
    count: usize
}
```

`transparent` требует ровно одно поле и не сочетается с `fmt` или строкой шаблона. `fmt = path` указывает на функцию, принимающую ссылки на все поля и `Formatter` последним аргументом.

## Атрибуты полей

| Атрибут | Эффект |
|---|---|
| `#[source]` | Поле возвращается из `source()`. Поддерживается `Option<E>`. |
| `#[from]` | Генерирует `From<FieldType>` для обёртки; подразумевает `#[source]` на том же поле. |
| `#[backtrace]` | Поле хранит `std::backtrace::Backtrace` (или `Option<Backtrace>`), доступный через интроспекцию ошибки, либо делегирует к бэктрейсу источника в сочетании с `#[source]`. |

Автовывод: поле с именем `source` автоматически считается источником, а поле типа `std::backtrace::Backtrace` (или `Option<Backtrace>`) распознаётся как бэктрейс без атрибута.

Enum принимают `#[error]` и `#[from]`/`#[source]`/`#[backtrace]` для каждого варианта:

```rust
use masterror::Error;

#[derive(Debug, Error)]
#[error("leaf failure")]
struct LeafError;

#[derive(Debug, Error)]
enum EnumError {
    #[error("unit failure")]
    Unit,
    #[error("{code}")]
    Code {
        code:  u16,
        #[source]
        cause: LeafError
    },
    #[error(transparent)]
    Wrapped(#[from] LeafError)
}
```

## `#[app_error(...)]` — конверсии в AppError

Описывает, как производная ошибка транслируется в `AppError`/`AppCode`. Опции: `kind` (обязательная), `code` (опциональная), `message` (флаг).

```rust
use masterror::{AppCode, AppError, AppErrorKind, Error};

#[derive(Debug, Error)]
#[error("missing flag: {name}")]
#[app_error(kind = AppErrorKind::BadRequest, code = AppCode::BadRequest, message)]
struct MissingFlag {
    name: &'static str
}

let app: AppError = MissingFlag { name: "feature" }.into();
assert!(matches!(app.kind, AppErrorKind::BadRequest));

let code: AppCode = MissingFlag { name: "other" }.into();
assert_eq!(code, AppCode::BadRequest);
```

- `kind = ...` выбирает `AppErrorKind`; генерирует `From<T> for AppError`.
- `code = ...` дополнительно генерирует `From<T> for AppCode`.
- `message` пробрасывает вывод `Display` как публичное сообщение; опустите его, чтобы сообщение осталось внутренним.

Enum выбирают отображение для каждого варианта, при этом derive всё равно генерирует единственную реализацию `From<Enum> for AppError`.

## `#[provide(...)]` — типизированная телеметрия

Предоставляет типизированный контекст через `std::error::Request` (nightly `error_generic_member_access`; компилируется автоматически при доступности). Поля `Option` регистрируют провайдер, только когда заполнены:

```rust
use masterror::{AppCode, AppErrorKind, Error};

#[derive(Clone, Debug, PartialEq, Eq)]
struct TelemetrySnapshot {
    name:  &'static str,
    value: u64
}

#[derive(Debug, Error)]
#[error("structured telemetry {snapshot:?}")]
#[app_error(kind = AppErrorKind::Service, code = AppCode::Service)]
struct StructuredTelemetryError {
    #[provide(ref = TelemetrySnapshot, value = TelemetrySnapshot)]
    snapshot: TelemetrySnapshot
}
```

Потребители извлекают снимок вызовом `std::error::request_ref::<TelemetrySnapshot>(&err)` на доменной ошибке.

## `#[derive(Masterror)]` — сквозные доменные ошибки

`#[derive(Masterror)]` генерирует `Display`, `std::error::Error`, `From<T> for masterror::Error` **и** таблицы транспортных отображений на этапе компиляции — всё конфигурируется одним атрибутом `#[masterror(...)]`:

```rust
use masterror::{
    AppCode, AppErrorKind, Error, Masterror, MessageEditPolicy, mapping::HttpMapping
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
    flag:    &'static str,
    attempt: Option<u64>,
    #[source]
    source:  Option<std::io::Error>
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
```

### Опции `#[masterror(...)]`

| Опция | Значение |
|---|---|
| `code = AppCode::...` | Публичный машиночитаемый код |
| `category = AppErrorKind::...` | Семантическая категория (определяет HTTP-статус) |
| `message` | Сделать отформатированный вывод `Display` безопасным публичным сообщением |
| `redact(message)` | Установить `MessageEditPolicy::Redact`, чтобы транспорты удаляли сообщение |
| `redact(fields("name" = hash, "card" = last4))` | Переопределить политики метаданных для полей: `hash`, `last4`, `redact`, `none` |
| `telemetry(expr, ...)` | Выражения, вычисляющиеся в `Option<masterror::Field>`; заполненные поля вставляются в `Metadata`. `telemetry()` — если полей нет |
| `map.grpc = <i32>` | Код статуса gRPC (совпадает с дискриминантами `tonic::Code`) |
| `map.problem = "<uri>"` | URI `type` по RFC 7807 |

### Генерируемые таблицы отображений

Для структур derive порождает ассоциированные константы; для enum — массив и срезы, агрегирующие отображения по вариантам:

| Форма | Константы |
|---|---|
| Структура | `T::HTTP_MAPPING: HttpMapping`, `T::GRPC_MAPPING: Option<GrpcMapping>`, `T::PROBLEM_MAPPING: Option<ProblemMapping>` |
| Enum | `T::HTTP_MAPPINGS: [HttpMapping; N]`, `T::GRPC_MAPPINGS: &'static [GrpcMapping]`, `T::PROBLEM_MAPPINGS: &'static [ProblemMapping]` |

Типы-дескрипторы живут в `masterror::mapping` (`HttpMapping::status()` выводит HTTP-код из категории; `GrpcMapping::status()` возвращает `i32`; `ProblemMapping::type_uri()` возвращает URI).

`#[from]`, `#[source]` и `#[backtrace]` продолжают работать под `#[derive(Masterror)]`; источники и захваченные бэктрейсы автоматически прикрепляются к результирующему `masterror::Error`, а источники, обёрнутые в `Arc`, переиспользуются без дополнительного клонирования.

## Выбор между derive-макросами

| Потребность | Используйте |
|---|---|
| `Display` + `source` + `From` в стиле thiserror | `#[derive(Error)]` |
| Плюс конверсия в `AppError`/`AppCode` | `#[derive(Error)]` + `#[app_error(...)]` |
| Типизированный контекст через `std::error::Request` | добавьте `#[provide(...)]` |
| Метаданные, политика редактирования, таблицы gRPC/problem+json | `#[derive(Masterror)]` + `#[masterror(...)]` |

---

См. также: [Начало работы](Начало-работы) · [Виды и коды ошибок](Виды-и-коды-ошибок) · [Контекст и метаданные](Контекст-и-метаданные) · [Миграция](Миграция)
