# Derive 매크로

`masterror`는 번들된 `masterror-derive` 크레이트를 통해 두 가지 파생을 제공합니다:

- **`#[derive(Error)]`** — `thiserror::Error`의 드롭인 대체품(동일한 `#[error]`, `#[from]`, `#[source]`, `#[backtrace]` 속성)으로, `#[app_error(...)]` 변환과 `#[provide(...)]` 텔레메트리로 확장되었습니다.
- **`#[derive(Masterror)]`** — 동일한 구문을 기반으로 `#[masterror(...)]`를 통해 메타데이터, 리덕션 정책, 전송 매핑 테이블과 함께 도메인 오류를 `masterror::Error`에 직접 연결합니다.

둘 다 루트에서 재수출됩니다: `use masterror::{Error, Masterror};`.

## `#[error("...")]` 템플릿

템플릿은 생성되는 `Display` 구현을 결정합니다. 플레이스홀더는 필드 이름(`{field}`), 튜플 인덱스(`{0}`) 또는 명시적 인수를 참조합니다. 파싱은 공유 `masterror-template` 크레이트가 처리하며 `thiserror` 의미론을 미러링합니다.

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

### 포매터 트레이트와 스펙

플레이스홀더는 전체 포매터 팔레트를 지원하며 — `{x:?}`, `{x:#?}`, `{x:x}`, `{x:#X}`, `{x:b}`, `{x:o}`, `{x:e}`, `{x:E}`, `{x:p}` — `{value:>8}`이나 `{ratio:.3}` 같은 디스플레이 전용 스펙은 그대로 전달됩니다. 프로그래밍 방식의 템플릿 검사를 위해 `masterror::error::template`은 `ErrorTemplate`, `TemplateFormatter`, `TemplateFormatterKind`를 노출합니다.

### 포맷 인수와 프로젝션

템플릿은 `self`에 대한 표현식과 `.field` 단축 표기를 통한 필드 프로젝션을 포함하여 이름 있는 인수와 위치 인수를 지원합니다:

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

### `transparent`와 `fmt = ...`

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

`transparent`는 정확히 하나의 필드를 요구하며 `fmt`나 템플릿 문자열과 함께 사용할 수 없습니다. `fmt = path`는 모든 필드에 대한 참조와 `Formatter`를 받는 함수를 가리킵니다.

## 필드 속성

| 속성 | 효과 |
|---|---|
| `#[source]` | 필드가 `source()`에서 반환됩니다. `Option<E>`가 지원됩니다. |
| `#[from]` | 래퍼에 대한 `From<FieldType>`을 생성합니다. 같은 필드에 `#[source]`를 함축합니다. |
| `#[backtrace]` | 필드가 오류 인트로스펙션을 통해 노출되는 `std::backtrace::Backtrace`(또는 `Option<Backtrace>`)를 보유하거나, `#[source]`와 결합되면 소스의 백트레이스에 위임합니다. |

추론: 문자 그대로 `source`라는 이름의 필드는 자동으로 소스로 취급되며, `std::backtrace::Backtrace`(또는 `Option<Backtrace>`) 타입의 필드는 속성 없이도 백트레이스로 인식됩니다.

열거형은 변형별 `#[error]`와 변형별 `#[from]`/`#[source]`/`#[backtrace]`를 지원합니다:

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

## `#[app_error(...)]` — AppError로의 변환

파생된 오류가 `AppError`/`AppCode`로 어떻게 변환되는지 기록합니다. 옵션: `kind`(필수), `code`(선택), `message`(플래그).

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

- `kind = ...`는 `AppErrorKind`를 선택하며 `From<T> for AppError`를 생성합니다.
- `code = ...`는 추가로 `From<T> for AppCode`를 생성합니다.
- `message`는 `Display` 출력을 공개 메시지로 전달합니다. 메시지를 내부용으로 유지하려면 생략하세요.

열거형은 변형별로 매핑을 선택하며, 파생은 여전히 단일 `From<Enum> for AppError`를 발행합니다.

## `#[provide(...)]` — 타입 기반 텔레메트리

`std::error::Request`(nightly `error_generic_member_access`; 사용 가능할 때 자동으로 컴파일에 포함됨)를 통해 타입 기반 컨텍스트를 노출합니다. `Option` 필드는 값이 채워졌을 때만 프로바이더를 등록합니다:

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

소비자는 도메인 오류에 대해 `std::error::request_ref::<TelemetrySnapshot>(&err)`로 스냅샷을 추출합니다.

## `#[derive(Masterror)]` — 엔드투엔드 도메인 오류

`#[derive(Masterror)]`는 `Display`, `std::error::Error`, `From<T> for masterror::Error` **그리고** 컴파일 타임 전송 매핑 테이블까지 생성하며, 모두 하나의 `#[masterror(...)]` 속성으로 구성합니다:

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

### `#[masterror(...)]` 옵션

| 옵션 | 의미 |
|---|---|
| `code = AppCode::...` | 공개 기계 판독 가능 코드 |
| `category = AppErrorKind::...` | 의미론적 범주 (HTTP 상태 결정) |
| `message` | 포매팅된 `Display` 출력을 안전한 공개 메시지로 노출 |
| `redact(message)` | 전송에서 메시지를 제거하도록 `MessageEditPolicy::Redact` 설정 |
| `redact(fields("name" = hash, "card" = last4))` | 필드별 메타데이터 정책 재정의: `hash`, `last4`, `redact`, `none` |
| `telemetry(expr, ...)` | `Option<masterror::Field>`로 평가되는 표현식. 값이 있는 필드는 `Metadata`에 삽입됩니다. 없을 때는 `telemetry()` 사용 |
| `map.grpc = <i32>` | gRPC 상태 코드 (`tonic::Code` 판별값과 일치) |
| `map.problem = "<uri>"` | RFC 7807 `type` URI |

### 생성되는 매핑 테이블

구조체의 경우 파생은 연관 상수를 발행하고, 열거형의 경우 변형별 매핑을 집계하는 배열과 슬라이스를 발행합니다:

| 형태 | 상수 |
|---|---|
| 구조체 | `T::HTTP_MAPPING: HttpMapping`, `T::GRPC_MAPPING: Option<GrpcMapping>`, `T::PROBLEM_MAPPING: Option<ProblemMapping>` |
| 열거형 | `T::HTTP_MAPPINGS: [HttpMapping; N]`, `T::GRPC_MAPPINGS: &'static [GrpcMapping]`, `T::PROBLEM_MAPPINGS: &'static [ProblemMapping]` |

디스크립터 타입은 `masterror::mapping`에 있습니다 (`HttpMapping::status()`는 종류에서 HTTP 코드를 파생하고, `GrpcMapping::status()`는 `i32`를 반환하며, `ProblemMapping::type_uri()`는 URI를 반환합니다).

`#[from]`, `#[source]`, `#[backtrace]`는 `#[derive(Masterror)]`에서도 계속 동작합니다. 소스와 캡처된 백트레이스는 결과 `masterror::Error`에 자동으로 첨부되며, `Arc`로 감싼 소스는 추가 복제 없이 재사용됩니다.

## 파생 선택 가이드

| 필요 | 사용 |
|---|---|
| thiserror 스타일의 `Display` + `source` + `From` | `#[derive(Error)]` |
| `AppError`/`AppCode`로의 변환까지 | `#[derive(Error)]` + `#[app_error(...)]` |
| `std::error::Request`를 통한 타입 기반 컨텍스트 | `#[provide(...)]` 추가 |
| 메타데이터, 리덕션 정책, gRPC/problem+json 테이블 | `#[derive(Masterror)]` + `#[masterror(...)]` |

---

함께 보기: [시작하기](시작하기) · [오류 종류와 코드](오류-종류와-코드) · [컨텍스트와 메타데이터](컨텍스트와-메타데이터) · [마이그레이션](마이그레이션)
