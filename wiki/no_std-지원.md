# no_std 지원

`masterror`는 Rust 표준 라이브러리 없이 빌드됩니다. 크레이트 루트는
`#![cfg_attr(not(feature = "std"), no_std)]`를 선언하며, 기본 `std` 기능이
임베디드/WASM 친화적 빌드와 여러분 사이에 있는 유일한 장벽입니다:

```toml
[dependencies]
masterror = { version = "0.28", default-features = false }
```

## alloc 필요

`masterror`는 `no_std`이지만 `no_alloc`은 **아닙니다**. 크레이트는
무조건 `extern crate alloc`을 선언하고 메시지, 메타데이터 및 소스 체인에
`Cow<'static, str>`, `String`, `Arc` 및 `BTreeMap`을 사용합니다. 타깃에
전역 할당자가 필요합니다. 순수 `core` 전용 환경은 지원되지 않습니다.

## `std` 없이 동작하는 것

프레임워크에 독립적인 코어 전체:

| 영역 | `no_std`에서 사용 가능 |
|---|---|
| 코어 타입 | `Error` / `AppError`, `AppResult`, `AppErrorKind`, `AppCode` |
| 메타데이터 | `Metadata`, `Field`, `FieldValue`, `FieldRedaction`, `field::*` 헬퍼 |
| 컨텍스트 | `Context`, `ResultExt::{ctx, context}` |
| 제어 흐름 | `ensure!`, `fail!` |
| 파생 | 모든 속성을 지원하는 `#[derive(Error)]`, `#[derive(Masterror)]` |
| 와이어 타입 | `ProblemJson`, `ErrorResponse`, `CODE_MAPPINGS`, `mapping_for_code` |
| 인트로스펙션 | `chain()`, `root_cause()`, `is`/`downcast`/`downcast_ref`/`downcast_mut`, `render_message()` |
| Serde | `alloc`과 함께 `serde` (와이어 타입의 JSON 직렬화) |

오류 소스는 **`core::error::Error`**를 통해 동작합니다: 크레이트는
`std::error::Error` 대신 `core::error::Error`(내부적으로 `CoreError`로
별칭됨)를 구현하고 소비하므로, `with_source(...)`, 소스 체인 및 다운캐스팅이
`no_std` 빌드에서 완전히 동작합니다.

```rust
use masterror::{AppCode, AppError, AppErrorKind, field};

let err = AppError::new(AppErrorKind::Timeout, "deadline exceeded")
    .with_field(field::u64("attempt", 3));

assert_eq!(err.code, AppCode::Timeout);
assert_eq!(err.metadata().len(), 1);
```

## `std`가 필요한 것

모든 런타임 통합은 기능 정의에서 명시적으로 `std`를 다시 활성화합니다.
`Cargo.toml` 기준:

- `tracing`, `metrics`, `backtrace`, `colored`
- `axum`, `actix`, `multipart`, `tonic`, `openapi`
- `serde_json`, `redis`, `validator`, `config`, `tokio`, `reqwest`,
  `teloxide`, `init-data`, `frontend`, `turnkey`

`backtrace`는 `std::backtrace::Backtrace`와 환경 변수 접근이 필요하고,
`colored`는 TTY 감지가 필요하며, 웹 및 클라이언트 통합은 그 자체가 `std`
전용인 호스트 크레이트가 필요합니다.

## CI 기능 매트릭스

`no_std` CI 잡(`.github/workflows/ci.yml`)은 모든 풀 리퀘스트와 `main`
푸시마다 다음 조합을 검사합니다:

| 잡 | 명령 | 검증 내용 |
|---|---|---|
| `bare` | `cargo check --no-default-features` | 진정한 `no_std` + `alloc` 빌드 |
| `std-only` | `cargo check --features std` | 기본 std 표면 |
| `tracing` | `cargo check --no-default-features --features tracing` | 단일 텔레메트리 기능의 독립 빌드 |
| `metrics` | `cargo check --no-default-features --features metrics` | metrics도 동일 |
| `colored` | `cargo check --no-default-features --features colored` | colored도 동일 |
| `all-features` | `cargo check --all-features` | 전체 기능 합집합 |

의미에 주의하세요: `bare` 잡만이 진정한 `no_std` 컴파일입니다.
`tracing = [..., "std"]`, `metrics = [..., "std"]` 및
`colored = [..., "std"]`는 전이적으로 `std`를 다시 활성화하므로, 해당 잡들은
기본 기능이 꺼져 있을 때 각 텔레메트리 기능이 자족적인지 검증할 뿐 —
텔레메트리가 표준 라이브러리 없이 동작한다는 뜻은 아닙니다. 텔레메트리가
필요하면 `std`가 필요합니다.

## 실용적 구성

전송에 독립적이고 `no_std` 호환을 유지하려는 라이브러리 크레이트:

```toml
[dependencies]
masterror = { version = "0.28", default-features = false }

[features]
std = ["masterror/std"]
```

바이너리 또는 서비스 크레이트는 `std`와 필요한 통합을 함께 켭니다:

```toml
[dependencies]
masterror = { version = "0.28", features = ["axum", "tracing", "metrics"] }
```

`AppErrorKind`, `AppCode` 및 와이어 타입이 `no_std` 코어에 있기 때문에,
도메인 크레이트는 오류를 분류하고 `ProblemJson` 페이로드까지 빌드할 수
있으며, HTTP 매핑은 서비스 크레이트에서만 이루어집니다 —
[모범 사례](모범-사례)를 참조하세요.

## 툴체인

크레이트는 `Cargo.toml`에서 `rust-version = "1.96"`으로 에디션 2024를
대상으로 합니다. `no_std` 소스 체인의 기반인 `core::error::Error`는
Rust 1.81부터 안정화되었으므로 nightly 기능은 전혀 사용되지 않습니다.

함께 보기: [기능 플래그](기능-플래그) · [시작하기](시작하기) · [모범 사례](모범-사례)
