<!--
SPDX-FileCopyrightText: 2025 RAprogramm <andrey.rozanov.vl@gmail.com>

SPDX-License-Identifier: MIT
-->

<div align="center">
  <img src="https://raw.githubusercontent.com/RAprogramm/masterror/main/images/materror.png" alt="masterror" width="600"/>
  <p><strong>Фреймворк-независимые типы ошибок для приложений</strong></p>

  <!-- ⚠️ GENERATED FILE: edit README.template.md and run `cargo build` to refresh README.md before publishing.
       CI packaging will fail if README.md is stale. -->

  [![Crates.io](https://img.shields.io/crates/v/masterror)](https://crates.io/crates/masterror)
  [![docs.rs](https://img.shields.io/docsrs/masterror)](https://docs.rs/masterror)
  [![Downloads](https://img.shields.io/crates/d/masterror)](https://crates.io/crates/masterror)
  ![MSRV](https://img.shields.io/badge/MSRV-1.90-blue)
  ![License](https://img.shields.io/badge/License-MIT%20or%20Apache--2.0-informational)
  [![codecov](https://codecov.io/gh/RAprogramm/masterror/graph/badge.svg?token=V9JQDTZLXH)](https://codecov.io/gh/RAprogramm/masterror)

  [![CI](https://github.com/RAprogramm/masterror/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/RAprogramm/masterror/actions/workflows/ci.yml?query=branch%3Amain)
  [![Hits-of-Code](https://hitsofcode.com/github/RAprogramm/masterror?branch=main)](https://hitsofcode.com/github/RAprogramm/masterror/view?branch=main)
  [![IMIR](https://raw.githubusercontent.com/RAprogramm/infra-metrics-insight-renderer/main/assets/badges/imir-badge-simple-public.svg)](https://github.com/RAprogramm/infra-metrics-insight-renderer)

  > 🇬🇧 [Read README in English](README.md)
  > 🇰🇷 [한국어 README](README.ko.md)

</div>

> [!IMPORTANT]
> Этот перевод был сгенерирован с помощью Claude. Если вы нашли ошибки или неточности, пожалуйста, [сообщите нам](https://github.com/RAprogramm/masterror/issues)!

---

## Содержание

- [Обзор](#обзор)
- [Особенности](#особенности)
- [Крейты рабочего пространства](#крейты-рабочего-пространства)
- [Флаги функций](#флаги-функций)
- [Установка](#установка)
- [Бенчмарки](#бенчмарки)
- [Покрытие кода](#покрытие-кода)
- [Быстрый старт](#быстрый-старт)
- [Расширенное использование](#расширенное-использование)
- [Ресурсы](#ресурсы)
- [Метрики](#метрики)
- [Лицензия](#лицензия)

---

## Обзор

`masterror` вырос из набора вспомогательных функций в рабочее пространство композируемых крейтов для построения согласованных, наблюдаемых поверхностей ошибок в сервисах Rust. Основной крейт остается независимым от фреймворков, в то время как флаги функций активируют транспортные адаптеры, интеграции и телеметрию без включения тяжеловесных зависимостей по умолчанию. Без использования `unsafe`, с зафиксированной минимальной версией Rust (MSRV), а derive-макросы позволяют вашим доменным типам контролировать редактирование и метаданные.

<div align="right">

<div align="right">
  <a href="#содержание">
    <img src="https://raw.githubusercontent.com/RAprogramm/masterror/main/images/masterror_go_to_top.png" alt="Go to top" width="50"/>
  </a>
</div>

</div>

---

## Особенности

- **Унифицированная таксономия.** `AppError`, `AppErrorKind` и `AppCode` моделируют доменные и транспортные задачи с консервативными отображениями HTTP/gRPC, готовыми подсказками для повторных попыток и аутентификации, а также вывода RFC7807 через `ProblemJson`.
- **Нативные derive-макросы.** `#[derive(Error)]`, `#[derive(Masterror)]`, `#[app_error]`, `#[masterror(...)]` и `#[provide]` встраивают пользовательские типы в `AppError`, передавая источники, бэктрейсы, провайдеры телеметрии и политики редактирования.
- **Типизированная телеметрия.** `Metadata` хранит структурированный контекст в формате ключ/значение (строки, целые числа, числа с плавающей точкой, длительности, IP-адреса и опциональный JSON) с контролем редактирования для каждого поля и конструкторами в `field::*`, чтобы логи оставались структурированными без ручного создания `String`-карт.
- **Транспортные адаптеры.** Опциональные функции предоставляют респондеры Actix/Axum, конверсии в `tonic::Status`, логирование WASM/браузера и генерацию схемы OpenAPI без загрязнения компактной сборки по умолчанию.
- **Проверенные интеграции.** Включите точечные отображения для `sqlx`, `reqwest`, `redis`, `validator`, `config`, `tokio`, `teloxide`, `multipart`, Telegram WebApp SDK и других — каждое из них транслирует библиотечные ошибки в таксономию с присоединенной телеметрией.
- **Готовые значения по умолчанию.** Модуль `turnkey` поставляет готовый к использованию каталог ошибок, вспомогательные конструкторы и инструментарий трассировки для команд, которые хотят получить согласованную базовую линию из коробки.
- **Типизированные макросы управления потоком.** `ensure!` и `fail!` прерывают выполнение функций с доменными ошибками без выделения памяти или форматирования на успешном пути.

<div align="right">

<div align="right">
  <a href="#содержание">
    <img src="https://raw.githubusercontent.com/RAprogramm/masterror/main/images/masterror_go_to_top.png" alt="Go to top" width="50"/>
  </a>
</div>

</div>

---

## Крейты рабочего пространства

| Крейт | Что предоставляет | Когда использовать |
| --- | --- | --- |
| [`masterror`](https://crates.io/crates/masterror) | Основные типы ошибок, конструкторы метаданных, транспорты, интеграции и прелюдия. | Крейты приложений, сервисы и библиотеки, которым нужна стабильная поверхность ошибок. |
| [`masterror-derive`](masterror-derive/README.md) | Процедурные макросы, поддерживающие `#[derive(Error)]`, `#[derive(Masterror)]`, `#[app_error]` и `#[provide]`. | Подключается автоматически через `masterror`; зависите напрямую только для работы с макросами. |
| [`masterror-template`](masterror-template/README.md) | Общий парсер шаблонов, используемый derive-макросами для анализа форматтеров. | Внутренняя зависимость; переиспользуйте, когда нужен парсер шаблонов в других местах. |

<div align="right">

<div align="right">
  <a href="#содержание">
    <img src="https://raw.githubusercontent.com/RAprogramm/masterror/main/images/masterror_go_to_top.png" alt="Go to top" width="50"/>
  </a>
</div>

</div>

---

## Флаги функций

Выбирайте только то, что вам нужно; по умолчанию все отключено.

- **Веб-транспорты:** `axum`, `actix`, `multipart`, `openapi`, `serde_json`.
- **Телеметрия и наблюдаемость:** `tracing`, `metrics`, `backtrace`.
- **Асинхронные интеграции и ввод/вывод:** `tokio`, `reqwest`, `sqlx`, `sqlx-migrate`, `redis`, `validator`, `config`.
- **Обмен сообщениями и боты:** `teloxide`, `telegram-webapp-sdk`.
- **Инструменты фронтенда:** `frontend` для логирования WASM/консоли браузера.
- **gRPC:** `tonic` для отправки ответов `tonic::Status`.
- **Всё включено:** `turnkey` для принятия готовой таксономии и вспомогательных функций.

Скрипт сборки поддерживает полный список функций ниже в синхронизации с `Cargo.toml`.

<div align="right">

<div align="right">
  <a href="#содержание">
    <img src="https://raw.githubusercontent.com/RAprogramm/masterror/main/images/masterror_go_to_top.png" alt="Go to top" width="50"/>
  </a>
</div>

</div>

---

## Установка

~~~toml
[dependencies]
masterror = { version = "0.24.19", default-features = false }
# или с функциями:
# masterror = { version = "0.24.19", features = [
#   "std", "axum", "actix", "openapi",
#   "serde_json", "tracing", "metrics", "backtrace",
#   "sqlx", "sqlx-migrate", "reqwest", "redis",
#   "validator", "config", "tokio", "multipart",
#   "teloxide", "telegram-webapp-sdk", "tonic", "frontend",
#   "turnkey", "benchmarks"
# ] }
~~~

<div align="right">

<div align="right">
  <a href="#содержание">
    <img src="https://raw.githubusercontent.com/RAprogramm/masterror/main/images/masterror_go_to_top.png" alt="Go to top" width="50"/>
  </a>
</div>

</div>

---

## Бенчмарки

Бенчмарки Criterion покрывают наиболее критичные пути конверсии, чтобы регрессии были видны до релиза. Запустите их локально с помощью:

~~~sh
cargo bench -F benchmarks --bench error_paths
~~~

Набор тестов выдает две группы:

- `context_into_error/*` продвигает фиктивную исходную ошибку с репрезентативными метаданными (строки, счетчики, длительности, IP-адреса) через `Context::into_error` в режимах с редактированием и без редактирования.
- `problem_json_from_app_error/*` использует результирующие значения `AppError` для построения полезных нагрузок RFC 7807 через `ProblemJson::from_app_error`, показывая, как редактирование сообщений и политики полей влияют на сериализацию.

Настройте флаги командной строки Criterion (например, `--sample-size 200` или `--save-baseline local`) после `--` для обмена пропускной способности на более точные доверительные интервалы при исследовании изменений.

<div align="right">

<div align="right">
  <a href="#содержание">
    <img src="https://raw.githubusercontent.com/RAprogramm/masterror/main/images/masterror_go_to_top.png" alt="Go to top" width="50"/>
  </a>
</div>

</div>

---

## Покрытие кода

[![codecov](https://codecov.io/gh/RAprogramm/masterror/branch/main/graph/badge.svg?token=V9JQDTZLXH)](https://app.codecov.io/gh/RAprogramm/masterror)

Отчеты о покрытии автоматически генерируются при каждом запуске CI и загружаются в [Codecov](https://app.codecov.io/gh/RAprogramm/masterror). Проект поддерживает высокое покрытие тестами во всех модулях для обеспечения надежности и раннего обнаружения регрессий.

<details>
  <summary><b>Визуализации покрытия</b></summary>

#### График Sunburst
Самый внутренний круг представляет весь проект, двигаясь наружу через папки к отдельным файлам. Размер и цвет указывают на количество операторов и процент покрытия.

[![Sunburst](https://codecov.io/gh/RAprogramm/masterror/branch/main/graphs/sunburst.svg?token=V9JQDTZLXH)](https://app.codecov.io/gh/RAprogramm/masterror)

#### Сеточное представление
Каждый блок представляет отдельный файл. Размер и цвет блока соответствуют количеству операторов и проценту покрытия.

[![Grid](https://codecov.io/gh/RAprogramm/masterror/branch/main/graphs/tree.svg?token=V9JQDTZLXH)](https://app.codecov.io/gh/RAprogramm/masterror)

#### Диаграмма Icicle
Иерархическое представление, начинающееся со всего проекта вверху, с детализацией через папки к отдельным файлам. Размер и цвет отражают количество операторов и покрытие.

[![Icicle](https://codecov.io/gh/RAprogramm/masterror/branch/main/graphs/icicle.svg?token=V9JQDTZLXH)](https://app.codecov.io/gh/RAprogramm/masterror)

</details>

<div align="right">

<div align="right">
  <a href="#содержание">
    <img src="https://raw.githubusercontent.com/RAprogramm/masterror/main/images/masterror_go_to_top.png" alt="Go to top" width="50"/>
  </a>
</div>

</div>

---

## Быстрый старт

<details>
  <summary><b>Создание ошибки</b></summary>

Создайте ошибку:

~~~rust
use masterror::{AppError, AppErrorKind, field};

let err = AppError::new(AppErrorKind::BadRequest, "Flag must be set");
assert!(matches!(err.kind, AppErrorKind::BadRequest));
let err_with_meta = AppError::service("downstream")
    .with_field(field::str("request_id", "abc123"));
assert_eq!(err_with_meta.metadata().len(), 1);

let err_with_context = AppError::internal("db down")
    .with_context(std::io::Error::new(std::io::ErrorKind::Other, "boom"));
assert!(err_with_context.source_ref().is_some());
~~~

С прелюдией:

~~~rust
use masterror::prelude::*;

fn do_work(flag: bool) -> AppResult<()> {
    if !flag {
        return Err(AppError::bad_request("Flag must be set"));
    }
    Ok(())
}
~~~

</details>

<div align="right">

<div align="right">
  <a href="#содержание">
    <img src="https://raw.githubusercontent.com/RAprogramm/masterror/main/images/masterror_go_to_top.png" alt="Go to top" width="50"/>
  </a>
</div>

</div>

---

## Расширенное использование

<details>
  <summary><b>Быстрый отказ без потери типизации</b></summary>

`ensure!` и `fail!` предоставляют типизированные альтернативы активно форматирующим хелперам `anyhow::ensure!`/`anyhow::bail!`. Они вычисляют выражение ошибки только когда проверка не проходит, поэтому успешные пути остаются без выделения памяти.

~~~rust
use masterror::{AppError, AppErrorKind, AppResult};

fn guard(flag: bool) -> AppResult<()> {
    masterror::ensure!(flag, AppError::bad_request("flag must be set"));
    Ok(())
}

fn bail() -> AppResult<()> {
    masterror::fail!(AppError::unauthorized("token expired"));
}

assert!(guard(true).is_ok());
assert!(matches!(guard(false).unwrap_err().kind, AppErrorKind::BadRequest));
assert!(matches!(bail().unwrap_err().kind, AppErrorKind::Unauthorized));
~~~

</details>

<details>
  <summary><b>Derive доменных ошибок и их отображение на транспорты</b></summary>

`masterror` поставляет нативные derive-макросы, чтобы ваши доменные типы оставались выразительными, в то время как крейт обрабатывает конверсии, телеметрию и редактирование за вас.

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

- `use masterror::Error;` вводит derive-макрос в область видимости.
- `#[from]` автоматически реализует `From<...>`, обеспечивая валидность форм-оберток.
- `#[error(transparent)]` требует обертки с одним полем, которая пробрасывает `Display`/`source` к внутренней ошибке.
- `#[app_error(kind = AppErrorKind::..., code = AppCode::..., message)]` отображает произведенную ошибку в `AppError`/`AppCode`. Опциональная часть `code = ...` генерирует конверсию `AppCode`, в то время как флаг `message` пробрасывает произведенный вывод `Display` как публичное сообщение вместо создания голой ошибки.
- `masterror::error::template::ErrorTemplate` парсит строки `#[error("...")]`, предоставляя литеральные и placeholder-сегменты, чтобы пользовательские derive-макросы могли быть реализованы без зависимости от `thiserror`.
- `TemplateFormatter` отражает обнаружение форматтера `thiserror`, поэтому существующие derive-макросы, которые полагались на шестнадцатеричные, указатели или экспоненциальные рендереры, продолжают компилироваться.
- Заполнители Display сохраняют свои необработанные спецификации формата через `TemplateFormatter::display_spec()` и `TemplateFormatter::format_fragment()`, чтобы сгенерированный код мог пробрасывать `:>8`, `:.3` и другие опции только для отображения без восстановления исходной строки.
- `TemplateFormatterKind` предоставляет трейт форматтера, запрошенный placeholder'ом, упрощая ветвление по запрошенному поведению рендеринга без ручного сопоставления каждого варианта enum.

</details>

<details>
  <summary><b>Присоединение телеметрии, политики редактирования и конверсий</b></summary>

`#[derive(Masterror)]` встраивает доменную ошибку в [`masterror::Error`], добавляет метаданные, политику редактирования и опциональные отображения на транспорты. Сопутствующий атрибут `#[masterror(...)]` отражает синтаксис `#[app_error]`, оставаясь явным относительно телеметрии и редактирования.

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

- `code` / `category` выбирают публичный [`AppCode`] и внутренний [`AppErrorKind`].
- `message` пробрасывает отформатированный вывод [`Display`] как безопасное публичное сообщение. Опустите, чтобы сохранить сообщение приватным.
- `redact(message)` переключает [`MessageEditPolicy`] на редактируемый на транспортной границе, `fields("name" = hash, "card" = last4)` переопределяет политики метаданных (`hash`, `last4`, `redact`, `none`).
- `telemetry(...)` принимает выражения, которые вычисляются в `Option<masterror::Field>`. Каждое заполненное поле вставляется в результирующие [`Metadata`]; используйте `telemetry()`, когда поля не присоединены.
- `map.grpc` / `map.problem` захватывают опциональные коды статуса gRPC (как `i32`) и URI `type` RFC 7807. Derive генерирует таблицы, такие как `MyError::HTTP_MAPPING`, `MyError::GRPC_MAPPING` и `MyError::PROBLEM_MAPPING` (или варианты слайсов для enum) для последующих интеграций.

Все привычные атрибуты уровня поля (`#[from]`, `#[source]`, `#[backtrace]`) по-прежнему учитываются. Источники и бэктрейсы автоматически присоединяются к сгенерированному [`masterror::Error`].

</details>

<details>
  <summary><b>Структурированные провайдеры телеметрии и отображения AppError</b></summary>

`#[provide(...)]` предоставляет типизированный контекст через `std::error::Request`, в то время как `#[app_error(...)]` записывает, как ваша доменная ошибка транслируется в `AppError` и `AppCode`. Derive отражает синтаксис `thiserror` и расширяет его опциональным распространением телеметрии и прямыми конверсиями в типы времени выполнения `masterror`.

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

Опциональная телеметрия появляется только когда присутствует, поэтому `None` не регистрирует провайдера. Собственные снимки все еще могут быть предоставлены как значения, когда вызывающая сторона запрашивает владение:

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

Enum поддерживают телеметрию и метаданные конверсии для каждого варианта. Каждый вариант выбирает свое собственное отображение `AppErrorKind`/`AppCode`, в то время как derive генерирует единственную реализацию `From<Enum>`:

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

По сравнению с `thiserror`, вы сохраняете привычную поверхность derive-макросов, получая структурированную телеметрию (`#[provide]`) и первоклассные конверсии в `AppError`/`AppCode` без ручного связующего кода.

</details>

<details>
  <summary><b>Полезные нагрузки Problem JSON и подсказки для повторных попыток/аутентификации</b></summary>

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

</details>

<div align="right">

<div align="right">
  <a href="#содержание">
    <img src="https://raw.githubusercontent.com/RAprogramm/masterror/main/images/masterror_go_to_top.png" alt="Go to top" width="50"/>
  </a>
</div>

</div>

---

## Ресурсы

- Изучите [вики по обработке ошибок](https://github.com/RAprogramm/masterror/wiki) для пошаговых руководств, сравнений с `thiserror`/`anyhow` и рецептов решения проблем.
- Просмотрите [документацию крейта на docs.rs](https://docs.rs/masterror) для деталей API, руководств по конкретным функциям и таблиц транспортов.
- Проверьте [`CHANGELOG.md`](CHANGELOG.md) для основных моментов релизов и примечаний по миграции.
- Ознакомьтесь с [RustManifest](https://github.com/RAprogramm/RustManifest) для стандартов разработки и лучших практик, которым следует этот проект.

<div align="right">

<div align="right">
  <a href="#содержание">
    <img src="https://raw.githubusercontent.com/RAprogramm/masterror/main/images/masterror_go_to_top.png" alt="Go to top" width="50"/>
  </a>
</div>

</div>

---

## Метрики

![Metrics](https://raw.githubusercontent.com/RAprogramm/infra-metrics-insight-renderer/main/metrics/masterror.svg)

<div align="right">

<div align="right">
  <a href="#содержание">
    <img src="https://raw.githubusercontent.com/RAprogramm/masterror/main/images/masterror_go_to_top.png" alt="Go to top" width="50"/>
  </a>
</div>

</div>

---

## Лицензия

MSRV: **1.90** · Лицензия: **MIT OR Apache-2.0** · Без `unsafe`

