Коротко: добей masterror по трём фронтам — типобезопасный контекст, наблюдаемость из коробки, и строгая «доменность». Тогда thiserror/anyhow будут выглядеть как милые ретро-игрушки.

Ниже — конкретный план, без воды.

# 1) Ядро типа ошибки

* **Единый `Error` с кодом, категорией и полями.**
  Коды ошибок доменные, человекочитаемые и стабильные: `USER_NOT_FOUND`, `DB_DEADLOCK`, `RATE_LIMITED`.
  Категории: `Bug`, `Config`, `Timeout`, `Upstream`, `Validation`, `Auth`, `Io`, `Db` и т.д.

* **Строгая модель метаданных.**
  Не строковые «context», а `BTreeMap<&'static str, Value>` с мини-типами: `Str(&'static str)`, `String`, `I64`, `U64`, `F64`, `Bool`, `Uuid`, `Ip`, `Duration`, `Json(serde_json::Value)`. Так проще фильтровать/редактировать.

* **Фичи:**
  `std`/`no_std` + `alloc`, `backtrace`, `serde`, `tracing`, `axum`, `actix`, `tonic`, `utoipa`, `problem_json`, `redaction`, `codespace` (генерация кодов из enum’ов). Всё без `unsafe`.

Компромисс: чуть сложнее типы метаданных, зато исчезает «stringly-typed context».

# 2) Типобезопасный контекст вместо `anyhow::Context`

Сделай `ResultExt` с контекстом, который:

* захватывает **исходную причину** как `source` без аллокации;
* добавляет **типизированные поля**;
* помечает **секреты** для редактирования.

```rust
use core::fmt;
use std::borrow::Cow;

pub trait ResultExt<T, E> {
    fn ctx<F>(self, f: F) -> Result<T, masterror::Error>
    where
        F: FnOnce() -> masterror::Context; // typed context builder
}

impl<T, E> ResultExt<T, E> for Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn ctx<F>(self, f: F) -> Result<T, masterror::Error>
    where
        F: FnOnce() -> masterror::Context,
    {
        self.map_err(|e| masterror::Error::from_source(e).with_context(f()))
    }
}
```

Пример использования:

```rust
use masterror::{Context, field as f};

fn load_user(id: i64) -> Result<User, masterror::Error> {
    repo::get_user(id).ctx(|| {
        // Builder is typed; no string concat.
        Context::new("USER_NOT_FOUND")
            .category("Db")
            .with(f::i64("user_id", id))
    })
}
```

Комментарий: *No string concatenation, structured fields, easy filtering.*

# 3) `derive` 2.0 сильнее `thiserror`

Сделай `#[derive(Masterror)]` с богатыми атрибутами:

* `#[error(code = "DB_DEADLOCK", category = "Db", http = "SERVICE_UNAVAILABLE")]`
* `#[error(message = "deadlock detected on {resource}")]`
* `#[error(redact(fields("token","password")))]`
* `#[error(telemetry(level = "WARN", event = "db.deadlock"))]`
* `#[error(map.grpc = "RESOURCE_EXHAUSTED")]`
* `#[error(map.problem = "about:blank", title = "Conflict", status = 409)]`

И автогенерация:

* `Display` из шаблона, но без паник на формат-ошибках.
* `serde` представления: «внешнее» (без секретов) и «внутреннее» (под флагом).
* OpenAPI/`utoipa` схемы для каждого варианта.
* Таблица соответствий code → http/grpc/problem-json.

Компромисс: derive сложнее в реализации, зато пользователю один атрибут — и всё сводится в систему.

# 4) Наблюдаемость: трассировка, метрики, корреляция

* **`tracing::span`/`event`** из коробки: при создании ошибки логируй `event!(level, code, category, fields…)` с `#[track_caller]` и `Backtrace` по фиче.
* **Correlation IDs**: поле `trace_id` подтягивается из `tracing` MDC, если есть.
* **Метрики**: опционально инкрементируй `error_total{code,category}`.

Компромисс: лёгкая зависимость на `tracing` под фичей; по умолчанию выключено.

# 5) Транспортные маппинги: HTTP/gRPC/Problem+JSON

* **Axum/Actix**: `IntoResponse` честно сериализует `application/problem+json` (RFC 7807), пряча секреты; http-статус берётся из атрибута или таблицы.
* **Tonic**: маппинг в `Status` с `.code()` и `.message()` без утечки приватных полей.
* **Кастомные трансформеры**: `impl TryFrom<Error> for MyApiError`.

Компромисс: немного glue-кода, но пользователь получает «одну кнопку» вместо рутины.

# 6) Политика бектрейсов и локаций

* По умолчанию без бектрейса. Включение через фичу или env `RUST_BACKTRACE=1` уважается.
* `#[track_caller]` на фабриках/`ctx` — точные файлы/строки без стоимости аллокаций.
* Маленькие стеки: `SmallVec` для цепочек причин.

Компромисс: бектрейс может стоить дорого; делай ленивым.

# 7) Редактирование секретов

* Атрибуты на полях и ключах контекста: `redact`, `hash`, `last4`.
* Сериализация во внешний мир всегда применяет политику редактирования.
* Отдельный `Debug` для внутреннего лога с явным флагом «я понимаю, что это PROD-лог».

Компромисс: больше кода в derive, зато безопасность по умолчанию.

# 8) Интеграции для бэкенда

* **SQLx/SeaORM**: конвертеры в доменные коды (`UNIQUE_VIOLATION` → `USER_ALREADY_EXISTS`).
* **Reqwest/Hyper**: адаптеры сетевых ошибок в `category = "Upstream"` с полями `status`, `endpoint`.
* **Redis/Kafka**: типовые маппинги таймаутов/брокера.

Компромисс: extra features, но 80% рутины исчезает.

# 9) CLI/линтеры/генераторы

* `cargo masterror check`:

  * проверка уникальности `code`;
  * список «немаппленных» ошибок, попавших в HTTP 500;
  * «лишние секретные поля» без редактирования.
* `cargo masterror gen-catalog`: Markdown/JSON-каталог кодов для техподдержки.
* `#[deny(masterror::missing_code)]` — легкий proc-macro-lint на уровне crate.

Компромисс: дополнительный бинарник, но DX вырастет.

# 10) Док-стандарты и стабильные контракты

* Требование: каждый `code` имеет краткое описание, рекомендуемое действие, уровень серьезности, и «кто виноват» (client/server/upstream).
* Стабильность кодов: семвер-проверка в `cargo masterror check`.

Компромисс: дисциплина, зато миграции предсказуемы.

---

## Минимальные примеры

### 1) Объявление доменной ошибки

```rust
use serde::{Serialize, Deserialize};
use masterror::Masterror;

#[derive(Debug, Serialize, Deserialize, Masterror)]
pub enum UserError {
    #[error(
        code = "USER_NOT_FOUND",
        category = "Db",
        http = "NOT_FOUND",
        message = "user {user_id} not found",
    )]
    NotFound { user_id: i64 },

    #[error(
        code = "USER_ALREADY_EXISTS",
        category = "Db",
        http = "CONFLICT",
        message = "user with email already exists",
        redact(fields("email"))
    )]
    AlreadyExists { email: String },
}
```

### 2) Использование с контекстом

```rust
use masterror::{ResultExt, field as f};

fn create_user(req: CreateUser) -> Result<User, masterror::Error> {
    repo::insert_user(&req).ctx(|| {
        masterror::Context::new("USER_CREATE_FAILED")
            .category("Db")
            .with(f::str("op", "insert"))
            .with(f::str("email", &req.email)) // will be redacted if marked
    })
}
```

### 3) Ответ API (axum)

```rust
#[cfg(feature = "axum")]
async fn handler(Json(req): Json<CreateUser>) -> Result<Json<User>, masterror::Error> {
    let user = create_user(req).ctx(|| masterror::Context::new("USER_CREATE"))?;
    Ok(Json(user))
}
// Axum IntoResponse takes care of RFC7807 with proper status and fields.
```

---

## Чем это сильнее thiserror/anyhow

* **thiserror**: шикарный derive, но «тупой» с точки зрения доменных кодов, транспортов, редактирования и телеметрии.
  У тебя: коды/категории/HTTP/GRPC/problem-json/редакция/typed context — всё из коробки.

* **anyhow**: сверх-удобный «мешок всего», но строковый контекст и нулевая доменность.
  У тебя: строгие поля, компонуемая политика, маппинги по таблице, стабильные коды для поддержки.

Компромисс: чуть выше порог входа → взамен консистентная прод-система.

---

## Производительность и безопасность

* Никаких `panic!/unwrap/expect`. Все фабрики возвращают `Result`.
* Аллокации по требованию: поля и сообщения ленивые; короткие пути — без `String`.
* Цепочки причин на `SmallVec<[Cause; 3]>`. Бектрейс ленивый.
* Zero-copy где уместно (`Cow<'static, str>` для message templates).
* Защита: редактирование по умолчанию для известных ключей (`password`, `token`, `secret`, `api_key`).

---

## План внедрения по шагам

1. Ввести ядро `Error` + `Context` + `ResultExt`.
2. Выпустить `derive 2.0` с кодами/категориями и базовыми маппингами.
3. Добавить интеграции `axum/actix/tonic` и `problem+json`.
4. Включить `tracing` события и `cargo masterror check`.
5. Подтянуть `utoipa` генерацию каталога ошибок.
6. Завести `no_std + alloc` и фичу `backtrace`.
7. Включить редактирование секретов и политику сериализации.

---

Если сделаешь это аккуратно и без шизо-магии в макросах, thiserror останется для pet-проектов, anyhow — для прототипов, а masterror станет стандартным «рабочим» для сервисов, где важны коды, телеметрия и безопасность. Именно там выигрывают не «удобные строки», а строгие контракты и наблюдаемость.

